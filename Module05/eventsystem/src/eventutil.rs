

// We're defining a trait, which is like a contract that says "Hey, I can do this thing!"
// In this case, the thing is listening to events of type T
pub trait EventListener<T> {
    fn on_event(&self, event: &T);
}

// Now we're defining a struct, which is like a box that holds some stuff.
// In this case, the stuff is a list of event listeners that can listen to events of type T.
pub struct EventSystem<T> {
// We're storing here in Boxes, which are like pointers to the listeners, because we don't know what type of events they are exactly.
    listeners: Vec<Box<dyn EventListener<T>>>,
}


    // This is a constructor, which is like a special method that creates a new EventSystem.
    // It returns a new EventSystem with an empty list of listeners.
impl<T> EventSystem<T> {
    pub fn new() -> Self {
        EventSystem {
            listeners: Vec::new()
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn EventListener<T>>) {
        self.listeners.push(listener);
    }

    // This is like sending a message to all our friends, saying "Hey, something happened!"
    pub fn send_event(&self, event: &T) {
        for listener in &self.listeners {
            listener.on_event(event);
        }
    }
}