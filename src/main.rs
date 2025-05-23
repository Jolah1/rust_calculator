use std::io;
fn main() {
    let result: f64;
    let mut op: String = String::new();

    println!("Hello, Welcome to Jolah's rust calculator!");

    println!("Please enter the first number: ");
    let x: f64 = input_parser();

    println!("Please enter the second number: ");
    let y: f64 = input_parser();

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation: ");

    io::stdin().read_line(&mut op).expect("Invalid input");
    let op: i32 = match op.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return;
        }
    };
    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        _ => {
            println!("Error: Invalid selection!");
            return;
        }
    }

    println!("The result is: {}", result);
    println!("Thank you for using Jolah's calculator!");
    println!("Have a great day!");
}
fn input_parser() -> f64 {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Invalid Input");
    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return f64::NAN;
        }
    };
    return x;
}
