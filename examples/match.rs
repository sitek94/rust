use std::cmp::Ordering;

fn main() {
    let x = 10;
    let y = 50;

    match x.cmp(&y) {
        Ordering::Less => println!("x is smaller than y"),
        Ordering::Greater => println!("x is greater than y"),
        Ordering::Equal => println!("x is equal to y"),
    }
}
