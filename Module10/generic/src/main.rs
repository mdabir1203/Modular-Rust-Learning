mod where_syntax;
mod sized_and_unsized;
mod associated_types;
mod operator_overloading;
mod higher_kind;

fn main() {
    println!("Rust Generics Playground:");

    println!("\nRunning Where Clause Example:");
    where_syntax::run_example();

    println!("\nRunning Dynamically Sized Types Example:");
    sized_and_unsized::run_example();

    println!("\nRunning Associated Types Example:");
    associated_types::run_example();

    println!("\nRunning Operator Overloading Example:");
    operator_overloading::run_example();

    println!("\nRunning Higher-Kinded Types Example:");
    higher_kind::run_example();
}
