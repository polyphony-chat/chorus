#[derive(Debug)]
pub struct Gateway {}

pub trait Observer {
    fn update(&self, data: &str);
}

pub struct GatewayEvent<'a> {
    observers: Vec<&'a dyn Observer>,
    test_content: String,
}

impl<'a> GatewayEvent<'a> {
    pub fn new(test_content: String) -> Self {
        Self {
            observers: Vec::new(),
            test_content,
        }
    }

    pub fn subscribe(&mut self, observable: &'a dyn Observer) {
        self.observers.push(observable)
    }

    pub fn unsubscribe(&mut self, observable: &'a dyn Observer) {
        if let Some(index) = self.observers.iter().position(|&o| o == observable) {
            self.observers.remove(index);
        }
    }

    pub fn update_data(&mut self, test_content: String) {
        self.test_content = test_content;
        self.notify();
    }

    pub fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.test_content);
        }
    }
}
