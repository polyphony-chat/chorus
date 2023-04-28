use crate::{api::WebSocketEvent, errors::ObserverError};

#[derive(Debug)]
pub struct Gateway {}

pub trait Observer<T: WebSocketEvent> {
    fn update(&self, data: &T);
}

pub struct GatewayEvent<'a, T: WebSocketEvent> {
    observers: Vec<&'a dyn Observer<T>>,
    pub event_data: T,
    pub is_observed: bool,
}

impl<'a, T: WebSocketEvent> GatewayEvent<'a, T> {
    fn new(event_data: T) -> Self {
        Self {
            is_observed: false,
            observers: Vec::new(),
            event_data,
        }
    }

    pub fn is_observed(&self) -> bool {
        self.is_observed
    }

    pub fn subscribe(&mut self, observable: &'a dyn Observer<T>) -> Option<ObserverError> {
        if self.is_observed {
            return Some(ObserverError::AlreadySubscribedError);
        }
        self.is_observed = true;
        self.observers.push(observable);
        None
    }

    pub fn unsubscribe(&mut self, observable: &'a dyn Observer<T>) {
        // .retain()'s closure retains only those elements of the vector, which have a different
        // pointer value than observable.
        self.observers.retain(|obs| !std::ptr::eq(*obs, observable));
        self.is_observed = !self.observers.is_empty();
        return;
    }

    fn update_data(&mut self, new_event_data: T) {
        self.event_data = new_event_data;
        self.notify();
    }

    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.event_data);
        }
    }
}

#[cfg(test)]
mod example {
    use super::*;
    use crate::api::types::GatewayResume;

    struct Consumer;
    impl Observer<GatewayResume> for Consumer {
        fn update(&self, data: &GatewayResume) {
            println!("{}", data.token)
        }
    }

    #[test]
    fn test_observer_behaviour() {
        let mut event = GatewayEvent::new(GatewayResume {
            token: "start".to_string(),
            session_id: "start".to_string(),
            seq: "start".to_string(),
        });

        let new_data = GatewayResume {
            token: "token_3276ha37am3".to_string(),
            session_id: "89346671230".to_string(),
            seq: "3".to_string(),
        };

        let consumer = Consumer;

        event.subscribe(&consumer);

        event.notify();

        event.update_data(new_data);

        let second_consumer = Consumer;

        match event.subscribe(&second_consumer) {
            None => assert!(false),
            Some(err) => println!("You cannot subscribe twice: {}", err),
        }
    }
}
