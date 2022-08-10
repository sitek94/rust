use std::io;

fn main() {
    let correct_pin: u32 = 1234;

    println!("Welcome in our Bank");
    println!("Please, enter your PIN");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    let input: u32 = input.trim().parse().expect("PIN must be a number");

    if input == correct_pin {
        println!("PIN correct")
    } else {
        println!("Wrong PIN!")
    }
}
