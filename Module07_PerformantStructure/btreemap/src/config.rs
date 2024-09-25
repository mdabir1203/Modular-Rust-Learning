pub struct Config {
    pub degree: usize,
}

impl Config {
    pub fn load() -> Self {
        Self {
            degree: 4, // Example value; adjust as needed
        }
    }
}
