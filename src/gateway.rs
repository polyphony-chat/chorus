#[derive(Debug)]
pub struct Gateway {}

pub trait Observer {
    fn update(&self, data: &str);
}

pub struct GatewayEvent<'a> {
    observers: Vec<&'a dyn Observer>,
    pub test_content: String,
    pub is_observed: bool,
}

impl<'a> GatewayEvent<'a> {
    fn new(test_content: String) -> Self {
        Self {
            is_observed: false,
            observers: Vec::new(),
            test_content,
        }
    }

    pub fn is_observed(&self) -> bool {
        self.is_observed
    }

    pub fn subscribe(&mut self, observable: &'a dyn Observer) {
        if self.is_observed {
            return;
        }
        self.is_observed = true;
        self.observers.push(observable)
    }

    pub fn unsubscribe(&mut self, observable: &'a dyn Observer) {
        self.observers.pop();
        self.is_observed = false;
        return;
    }

    fn update_data(&mut self, test_content: String) {
        self.test_content = test_content;
        self.notify();
    }

    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.test_content);
        }
    }
}
