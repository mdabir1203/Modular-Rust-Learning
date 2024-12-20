pub trait Calculator {
    type Operand;
    type Result;

    fn calculate(&self, a: Self::Operand, b: Self::Operand) -> Self::Result;
    fn square(&self, a: Self::Operand) -> Self::Result;
}

pub struct Adder;

impl Calculator for Adder {
    type Operand = i32;
    type Result = i32;

    fn calculate(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn square(&self, a: i32) -> i32 {
        a * a
    }
}

pub fn run_example() {
    let adder = Adder;
    let sum = adder.calculate(3, 4);
    println!("3 + 4 = {}", sum); // Prints: 3 + 4 = 7

    let square = adder.square(5);
    println!("5 squared = {}", square); // Prints: 5 squared = 25
}
