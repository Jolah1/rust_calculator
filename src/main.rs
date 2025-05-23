use std::io;
fn main() {
    let mut x: String = String::new();
    let mut y: String = String::new();

    println!("Hello, Welcome to Jolah's rust calculator!");

    println!("Please enter the first number: ");
    io::stdin().read_line(&mut x).expect("Invalid input");

    println!("Please enter the second number: ");
    io::stdin().read_line(&mut y).expect("Invalid input");
}
