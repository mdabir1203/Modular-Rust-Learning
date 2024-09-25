use std::fmt::Display;

// Step 1: Define a Contract (Trait)
pub trait LogStrategy {
    fn log(&self, message: &str);
}

// Step 2: Create a Concrete Implementation
pub struct ConsoleLog;

impl LogStrategy for ConsoleLog {
    fn log(&self, message: &str) {
        println!("{}", message);
    }
}

// Step 3: Define a Generic Logger
pub struct Logger<T: LogStrategy> {
    strategy: T,
}

// Step 4: Implement Logger Methods
impl<T: LogStrategy> Logger<T> {
    pub fn new(strategy: T) -> Self {
        Logger { strategy }
    }

    pub fn log<M: Display>(&self, message: M) {
        self.strategy.log(&format!("{}", message));
    }
}