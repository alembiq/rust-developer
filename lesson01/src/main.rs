use std::env;
use std::io;

fn main() {
    let mut arguments = env::args();

    let _program_name = arguments.next();
    let prefix = arguments.next();
    let prefix = prefix.unwrap_or("Hello".to_string());

    println!("Enter your name:");
    let mut name = String::new();
    // Read the user input and store it in the 'name' variable
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    println!("{prefix}, {}!", name);
}
