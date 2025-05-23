use std::io;
fn main() {
    let mut result: f64;
    let mut y_or_n: String = String::new();
    let ops: [fn(f64, f64) -> f64; 5] = [add, subtract, multiply, divide, mod_];

    loop {
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

        if op > 5 || op < 1 {
            println!("Error: Invalid selection!");
            return;
        }

        result = ops[(op - 1) as usize](x, y);

        println!("The result is: {}", result);
        println!("continue? (y/n): ");
        io::stdin().read_line(&mut y_or_n).expect("Invalid Input");

        if y_or_n.trim() == "n" {
            println!("Thank you for using Jolah's calculator!");
            println!("Have a great day!");
            break;
        } else if y_or_n.trim() != "y" {
            println!("Invalid input!");
            println!("Thank you for using Jolah's calculator!");
            println!("Have a great day!");
            break;
        } else {
            y_or_n.clear();
        }
    }
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

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

fn divide(x: f64, y: f64) -> f64 {
    x / y
}

fn mod_(x: f64, y: f64) -> f64 {
    x % y
}
