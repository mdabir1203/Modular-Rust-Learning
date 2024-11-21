use my_macro::make_function; // Import the macro from the my_macro crate

make_function!(generated_function); // This will generate a function named `generated_function`

fn main() {
    println!("{}", generated_function()); // Call the generated function
}