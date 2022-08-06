use colored::Colorize;

fn main() {
    println!("{}", "this is blue".blue());

    println!(
        "{} {} {}",
        "or use".cyan(),
        "any".italic().yellow(),
        "string type".cyan()
    );

    println!("{}", "this is bold and red".bold().red());
}
