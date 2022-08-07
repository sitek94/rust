use std::io;

fn main() {
    println!("What's your name?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin!");

    let input: &str = input.trim();

    println!("Hi {}, nice to meet you!", input);
}
