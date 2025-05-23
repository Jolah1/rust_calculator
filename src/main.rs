use std::io;
fn main() {
    let result: f64;

    println!("Hello, Welcome to Jolah's rust calculator!");

    println!("Please enter the first number: ");
    let x: f64 = input_parser();

    if f64::is_nan(x) {
        println!("Invalid input!");
        return;
    }

    println!("Please enter the second number: ");
    let y: f64 = input_parser();

    if f64::is_nan(y) {
        println!("Invalid input!");
        return;
    }
    println!("Please select the operation you want to perform:");

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("(5) Modulus");
    println!("Select the number associated with the desired operation: ");

    let op: f64 = input_parser();

    if f64::is_nan(op) {
        println!("Invalid input!");
        return;
    }

    let op: i32 = op as i32;

    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        5 => result = x % y,
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
            return f64::NAN;
        }
    };
    return x;
}
