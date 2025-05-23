use std::io;
fn main() {
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut op: String = String::new();
    let result: i32;
    let mut op: String = String::new();

    println!("Hello, Welcome to Jolah's rust calculator!");

    println!("Please enter the first number: ");
    io::stdin().read_line(&mut x).expect("Invalid input");

    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return;
        }
    };

    println!("Please enter the second number: ");
    io::stdin().read_line(&mut y).expect("Invalid input");

    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return;
        }
    };

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
