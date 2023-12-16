//! Defines voice raw udp socket handling

use std::{net::SocketAddr, sync::Arc};

use log::{info, warn};
use tokio::net::UdpSocket;

use discortp::{
    demux::{demux, Demuxed},
    discord::{IpDiscovery, IpDiscoveryPacket, IpDiscoveryType, MutableIpDiscoveryPacket},
    Packet,
};

/// Handle to a voice udp connection
///
/// Can be safely cloned and will still correspond to the same connection.
#[derive(Debug, Clone)]
pub struct UdpHandle {
    /// Ip discovery data we received on init
    pub ip_discovery: IpDiscovery,
    socket: Arc<UdpSocket>,
}

#[derive(Debug)]
pub struct UdpHandler {
    socket: Arc<UdpSocket>,
}

impl UdpHandler {
    pub async fn spawn(url: SocketAddr, ssrc: u32) -> UdpHandle {
        // Bind with a port number of 0, so the os assigns this listener a port
        let udp_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        udp_socket.connect(url).await.unwrap();

        // First perform ip discovery
        let ip_discovery = IpDiscovery {
            pkt_type: IpDiscoveryType::Request,
            ssrc,
            length: 70,
            address: Vec::new(),
            port: 0,
            payload: Vec::new(),
        };

        let mut buf: Vec<u8> = Vec::new();

        let size = IpDiscoveryPacket::minimum_packet_size() + 64;

        for _i in 0..size {
            buf.push(0);
        }

        // TODO: Make this not panic everything
        let mut ip_discovery_packet =
            MutableIpDiscoveryPacket::new(&mut buf).expect("Mangled ip discovery packet");

        ip_discovery_packet.populate(&ip_discovery);

        let data = ip_discovery_packet.packet();

        info!("VUDP: Sending Ip Discovery {:?}", &data);

        udp_socket.send(data).await.unwrap();

        info!("VUDP: Sent packet discovery request");

        // Handle the ip discovery response
        let receieved_size = udp_socket.recv(&mut buf).await.unwrap();
        info!(
            "VUDP: Receiving messsage: {:?} - (expected {} vs real {})",
            buf.clone(),
            size,
            receieved_size
        );

        let receieved_ip_discovery = IpDiscoveryPacket::new(&buf).unwrap();

        info!(
            "VUDP: Received ip discovery!!! {:?}",
            receieved_ip_discovery
        );

        let socket = Arc::new(udp_socket);

        let mut handler = UdpHandler {
            socket: socket.clone(),
        };

        // Now we can continuously check for messages in a different task
        tokio::spawn(async move {
            handler.listen_task().await;
        });

        let ip_discovery = IpDiscovery {
            pkt_type: receieved_ip_discovery.get_pkt_type(),
            length: receieved_ip_discovery.get_length(),
            ssrc: receieved_ip_discovery.get_ssrc(),
            address: receieved_ip_discovery.get_address(),
            port: receieved_ip_discovery.get_port(),
            payload: Vec::new(),
        };

        return UdpHandle {
            ip_discovery,
            socket,
        };
    }

    /// The main listen task;
    ///
    /// Receives udp messages and parses them.
    pub async fn listen_task(&mut self) {
        loop {
            let mut buf: Vec<u8> = Vec::new();

            // FIXME: is there a better way to do this?
            for _i in 0..1_000 {
                buf.push(0);
            }
            let msg = self.socket.recv(&mut buf).await;
            if let Ok(size) = msg {
                self.handle_message(&buf[0..size]).await;
                continue;
            }

            warn!("VUDP: Voice UDP is broken, closing connection");
            break;
        }
    }

    /// Handles a message buf
    async fn handle_message(&self, buf: &[u8]) {
        info!("VUDP: Received messsage");

        let parsed = demux(buf);

        match parsed {
            Demuxed::Rtp(rtp) => {
                let data = buf[11..buf.len()].to_vec();
                info!("VUDP: Parsed packet as rtp! {:?}; data: {:?}", rtp, data);
            }
            Demuxed::Rtcp(rtcp) => {
                info!("VUDP: Parsed packet as rtcp! {:?}", rtcp);
            }
            Demuxed::FailedParse(e) => {
                warn!("VUDP: Failed to parse packet: {:?}", e);
            }
            Demuxed::TooSmall => {
                unreachable!()
            }
        }
    }
}
