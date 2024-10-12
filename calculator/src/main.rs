use std::io;

fn main() {
    let mut operation = String::new();
    let mut number1 = String::new();
    let mut number2 = String::new();

    println!("Please enter the operation you want to perform (+, -, *, /):");
    io::stdin()
        .read_line(&mut operation)
        .expect("Not able to read!");

    println!("Please enter the first number:");
    io::stdin()
        .read_line(&mut number1)
        .expect("Not able to read!");

    println!("Please enter the second number:");
    io::stdin()
        .read_line(&mut number2)
        .expect("Not able to read!");

    // Use `trim()` to clean input and handle it as a string
    let operator = operation.trim(); // Keep this as a string for matching
    let num1: i32 = number1.trim().parse().expect("Please enter a valid number");
    let num2: i32 = number2.trim().parse().expect("Please enter a valid number");
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0 {
                num1 / num2
            } else {
                println!("Error: Division by zero");
                return;
            }
        }
        _ => {
            println!("Invalid operator. Please enter one of +, -, *, /.");
            return;
        }
    };

    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}
