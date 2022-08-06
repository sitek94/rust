use std::io;

fn main() {
    println!("What's your name?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin!");

    let input_string = input.to_string();
    let input_trimmed = input_string.trim();

    println!("Hi {}, nice to meet you!", input_trimmed);
}
