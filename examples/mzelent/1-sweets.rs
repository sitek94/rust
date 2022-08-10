use std::io;

fn main() {
    println!("How many students are in your class? (including you)");

    let mut students = String::new();

    io::stdin()
        .read_line(&mut students)
        .expect("Failed to read from stdin!");

    let students: u32 = students.trim().parse().expect("Enter number!");

    println!("How many 🍬 have you got?");

    let mut sweets = String::new();

    io::stdin()
        .read_line(&mut sweets)
        .expect("Failed to read from stdin!");

    let sweets: u32 = sweets.trim().parse().expect("Enter number!");

    let sweets_per_student: u32 = sweets / students;
    let sweets_left = sweets - students * sweets_per_student;

    println!("Each student is going to get {sweets_per_student}🍬");
    println!("{sweets_left}🍬 is going to be left for you :)");
}
