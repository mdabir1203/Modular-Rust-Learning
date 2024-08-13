pub mod eventutil;

use eventutil::{EventSystem, EventListener};

struct PrintListener;
impl EventListener<String> for PrintListener {
    fn on_event(&self, event: &String) {
        println!("Received event: {}", event);
    }
}

pub struct CountListener {
    count: std::cell::Cell<usize>,
}

impl EventListener<String> for CountListener {
    fn on_event(&self, _: &String) {
        let current = self.count.get();
        self.count.set(current + 1);
        println!("Event count: {}", self.count.get());
    }
}


fn main() {
    let mut event_system = EventSystem::new();

    event_system.add_listener(Box::new(PrintListener));
    event_system.add_listener(Box::new(CountListener { count: std::cell::Cell::new(0) }));

    event_system.send_event(&"hocche na".to_string());
    event_system.send_event(&"hoye gese".to_string());
    event_system.send_event(&"hobe".to_string());
}