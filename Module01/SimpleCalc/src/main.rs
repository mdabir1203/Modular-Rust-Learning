use std::io;

/// Reads a number from the user's input.
///
/// # Returns
///
/// A `f64` representing the number entered by the user.
///
/// # Panics
///
/// Panics if the user input is not a valid number.
fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

/// Performs a calculation based on the given operator and numbers.
///
/// # Arguments
///
/// * `operator` - The operator to use for the calculation.
/// * `num1` - The first number.
/// * `num2` - The second number.
///
/// # Returns
///
/// A `String` containing the result of the calculation or an error message.
fn calculate(operator: &str, num1: f64, num2: f64) -> String {
    match operator {
        "+" => format!("Result: {}", num1 + num2),
        "-" => format!("Result: {}", num1 - num2),
        "*" => format!("Result: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                format!("Result: {}", num1 / num2)
            } else {
                "Cannot divide by zero".to_string()
            }
        }
        _ => "Invalid operation".to_string(),
    }
}

fn main() {
    println!("Welcome to crazy cure\n");

    println!("Enter the first number :");
    let num1 = read_number();

    println!("Enter the second number :");
    let num2 = read_number();

    println!("Choose an operation: +, -, *, /");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    let result = calculate(operator.trim(), num1, num2);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_read_number() {
    //     let _input = "42\n";
    //     let expected = 42.0;
    //     let result = read_number();
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn test_calculate_addition() {
        let operator = "+";
        let num1 = 10.0;
        let num2 = 5.0;
        let expected = "Result: 15".to_string();
        let result = calculate(operator, num1, num2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_subtraction() {
        let operator = "-";
        let num1 = 10.0;
        let num2 = 5.0;
        let expected = "Result: 5".to_string();
        let result = calculate(operator, num1, num2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_multiplication() {
        let operator = "*";
        let num1 = 10.0;
        let num2 = 5.0;
        let expected = "Result: 50".to_string();
        let result = calculate(operator, num1, num2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_division() {
        let operator = "/";
        let num1 = 10.0;
        let num2 = 5.0;
        let expected = "Result: 2".to_string();
        let result = calculate(operator, num1, num2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_division_by_zero() {
        let operator = "/";
        let num1 = 10.0;
        let num2 = 0.0;
        let expected = "Cannot divide by zero".to_string();
        let result = calculate(operator, num1, num2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_invalid_operation() {
        let operator = "%";
        let num1 = 10.0;
        let num2 = 5.0;
        let expected = "Invalid operation".to_string();
        let result = calculate(operator, num1, num2);
        assert_eq!(result, expected);
    }
}
