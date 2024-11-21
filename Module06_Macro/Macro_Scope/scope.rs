macro_rules! define_and_print {
    ($var:ident) => {
        let $var = 42;
        println!("Inside macro: {}", $var);
    };
}

fn main() {
    let var = 10;
    define_and_print!(new_var); // remember to use new variable name
    println!("Outside macro: {}", var);  // Prints 10, not 42
}