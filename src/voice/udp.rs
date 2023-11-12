//! Defines voice raw udp socket handling

use std::net::SocketAddr;

use log::{info, warn};
use tokio::net::UdpSocket;

use discortp::{
    discord::{IpDiscovery, IpDiscoveryPacket},
    MutablePacket, Packet,
};

#[derive(Debug)]
pub struct UdpHandler {
    url: SocketAddr,
    socket: UdpSocket,
}

impl UdpHandler {
    pub async fn new(url: SocketAddr, ssrc: u32) -> IpDiscovery {
        // Bind with a port number of 0, so the os assigns this listener a port
        let udp_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        udp_socket.connect(url).await.unwrap();

        // First perform ip discovery
        let ip_discovery = discortp::discord::IpDiscovery {
            pkt_type: discortp::discord::IpDiscoveryType::Request,
            ssrc,
            length: 70,
            address: Vec::new(),
            port: 0,
            payload: Vec::new(),
        };

        let mut buf: Vec<u8> = Vec::new();

        let size = IpDiscoveryPacket::minimum_packet_size() + 64;

        // wtf
        for _i in 0..size {
            buf.push(0);
        }

        let mut ip_discovery_packet =
            discortp::discord::MutableIpDiscoveryPacket::new(&mut buf).expect("FUcking kill me");

        ip_discovery_packet.populate(&ip_discovery);

        let data = ip_discovery_packet.packet();

        info!("VUDP: Sending Ip Discovery {:?}", &data);

        udp_socket.send(&data).await.unwrap();

        info!("VUDP: Sent packet discovery request");

        // Handle the ip discovery response
        let receieved_size = udp_socket.recv(&mut buf).await.unwrap();
        info!(
            "VUDP: Receiving messsage: {:?} - (expected {} vs real {})",
            buf.clone(),
            size,
            receieved_size
        );

        let receieved_ip_discovery = discortp::discord::IpDiscoveryPacket::new(&buf).unwrap();

        info!(
            "VUDP: Received ip discovery!!! {:?}",
            receieved_ip_discovery
        );

        let mut handler = UdpHandler {
            url,
            socket: udp_socket,
        };

        // Now we can continuously check for messages in a different task
        tokio::spawn(async move {
            handler.listen_task().await;
        });

        return IpDiscovery {
            pkt_type: receieved_ip_discovery.get_pkt_type(),
            length: receieved_ip_discovery.get_length(),
            ssrc: receieved_ip_discovery.get_ssrc(),
            address: receieved_ip_discovery.get_address(),
            port: receieved_ip_discovery.get_port(),
            payload: Vec::new(),
        };
    }

    /// The main listen task;
    ///
    /// Receives udp messages and parses them.
    pub async fn listen_task(&mut self) {
        loop {
            let mut buf: Vec<u8> = Vec::new();

            let size = IpDiscoveryPacket::minimum_packet_size() + 64;

            // wtf
            for _i in 0..size {
                buf.push(0);
            }
            let msg = self.socket.recv(&mut buf).await;
            if let Ok(size) = msg {
                info!("VUDP: Receiving messsage: {:?} - {}", buf.clone(), size);
                self.handle_message(&buf[0..size]).await;
                continue;
            }

            warn!("VUDP: Voice UDP is broken, closing connection");
            break;
        }
    }

    /// Handles a message buf
    async fn handle_message(&self, buf: &[u8]) {
        info!("VUDP: Received messsage {:?}", buf);
    }
}
