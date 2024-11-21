macro_rules! sum_list {
    () => (0);
    ($x:expr) => ($x); // ($x:expr)-> captures first expression
    ($x:expr, $($rest:tt)*) => (
        $x + sum_list!($($rest)*) // here it recursively checks the other
    );
}

fn main() {
    let result = sum_list!(5, 8, 7, 11); //    1 + sum_list!(2, 3, 4)
    println!("Sum: {}", result);  // Outputs: Sum: 10
}