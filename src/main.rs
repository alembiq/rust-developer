use std::env;
use std::env::Args;
use std::io;

fn main() {
    let mut arguments: Args = env::args();

    let _program_name: Option<String> = arguments.next();
    let prefix: Option<String> = arguments.next();
    let prefix: String = prefix.unwrap_or("Hello".to_string());

    // Print a message to ask for the user's name
    println!("Enter your name:");

    // Create a new String to store the user input
    let mut name = String::new();

    // Read the user input and store it in the 'name' variable
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

        // Trim the trailing newline character from the input
    let name = name.trim();

    // Print a greeting message with the user's name
    println!("{prefix}, {}!", name);
}
