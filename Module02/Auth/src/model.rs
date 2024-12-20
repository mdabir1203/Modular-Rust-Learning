use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String, 
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self {
        Claims { sub, exp }
    }

    pub fn exp(&self) -> usize {
        self.exp
    }

}