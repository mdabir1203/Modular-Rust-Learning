macro_rules! create_function {
    ($func_name:ident, $body:expr) => {
        fn $func_name() {
            println!("Executing function {}", stringify!($func_name));
            $body;
        }
    };
}

create_function!(example_func, { 
    println!("This is the function body");
});

fn main() {
    example_func();
}