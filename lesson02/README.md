Description:

In this (still very simple) exercise, you'll be using Rust's string manipulation capabilities. Here's what you need to do:

    Setting up the Crate:
        Add the slug crate to your Cargo project to help with the slugify feature. To do this, open your Cargo.toml file and under the [dependencies] section, add: slug = "latest_version". (Replace "latest_version" with the most recent version number from crates.io, which is 0.1.4)
        Once added, you can use it in your project by adding use slug::slugify; at the top of your main Rust file. View the crate's documentation to see how to use it: https://docs.rs/slug/0.1.4/slug/

    Read Input:
        Read a string from the standard input.

    Parse CLI Arguments:
        Based on the provided CLI argument, the program should modify the text's behavior. Use the std::env::args() method to collect CLI arguments:

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", args[0]);
}

Note that the .len() and .is_empty() methods are available on Vector<String> to help you figure out, if you received the necessary parameters.

    Transmute Text:
        If the argument is lowercase, convert the entire text to lowercase.
        If the argument is uppercase, convert the entire text to uppercase.
        If the argument is no-spaces, remove all spaces from the text.
        If the argument is slugify, convert the text into a slug (a version of the text suitable for URLs) using the slug crate.

For one bonus point, try making two additional transformations of your own.

    Print Result:
        Print the transmuted text back to the user.

Hint: For string manipulations, the Rust standard library provides handy methods like:

    to_lowercase()
    to_uppercase()
    replace(" ", "")
