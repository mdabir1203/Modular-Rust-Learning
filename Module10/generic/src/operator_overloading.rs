use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn run_example() {
    let v1 = Vector2D { x: 1, y: 2 };
    let v2 = Vector2D { x: 3, y: 4 };

    let sum = v1 + v2;
    println!("Sum: {:?}", sum); // Prints: Sum: Vector2D { x: 4, y: 6 }

    let v3 = Vector2D { x: 10, y: 20 };
    let v4 = Vector2D { x: 5, y: 15 };

    let diff = v3 - v4;
    println!("Difference: {:?}", diff); // Prints: Difference: Vector2D { x: 5, y: 5 }
}
