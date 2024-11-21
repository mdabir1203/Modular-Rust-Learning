# Macro usage in Rust 

1. linear_Macro/Macro_linear.rs
- This file contains the definition of the sum_list macro, which computes the sum of a list of numbers.

## Key Points:
- The macro uses pattern matching to handle different cases: no arguments, a single argument, and multiple arguments.
- It recursively sums the numbers provided.

# Procedural Macro

- It enables the library as a procedural macro.
- It includes dependencies on quote and syn, which are commonly used for writing macros in Rust.

# Scope Macro
- This code defines a macro define_and_print that creates a new variable and prints its value. In the main function, it demonstrates the use of this macro while also showing the scope of variables.