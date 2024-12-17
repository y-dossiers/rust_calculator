use std::io;

fn main() {
    println!("Welcome to the Simple Calculator!");
    println!("Insert Start");

    // Read user input for the operation
    let mut return_home = String::new();
    io::stdin()
        .read_line(&mut return_home)
        .expect("Failed to read return_home");

    let return_home = return_home.trim(); // Remove newline character

    // Get first number
    let num1 = read_number("Enter the first number: ");

    let answer = 0;


    if answer == 1 {
        println!("Select an operation: +, -, *, /");

        // Read user input for the operation
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read operation");

        let operation = operation.trim(); // Remove newline character

        // Get first number
        let num1 = read_number("Enter the first number: ");

        // Get second number
        let num2 = read_number("Enter the second number: ");

        // Perform calculation based on the operation
        let result = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Error: Division by zero is not allowed.");
                    return;
                }
            }
            _ => {
                println!("Invalid operation. Please use +, -, *, or /.");
                return;
            }
        };

        // Print the result
        println!("The result of {} {} {} = {}", num1, operation, num2, result);
    }
    else {
        println!("Good Bye !");
    }
}

// Helper function to read a number
fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
