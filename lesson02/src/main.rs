use slug::slugify;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "Usage: {} lowercase/uppercase/no-space/slugify/reverse/capitalize",
            args[0]
        );
        return;
    };

    println!("What should I transform? ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    let output = match &args[1][..] {
        "lowercase" => input.to_lowercase(),
        "uppercase" => input.to_uppercase(),
        "no-space" => input.replace(' ', ""),
        "slugify" => slugify(input),
        "reverse" => input.chars().rev().collect(),
        "capitalize" => format!("{}{}", &input[..1].to_uppercase(), &input[1..]),
        _ => {
            println!("Invalid transformation");
            return;
        }
    };

    println!("{}: {}", args[1], output);
}
