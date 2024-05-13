use slug::slugify;
use std::env;
use std::error::Error;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "Usage: {} lowercase/uppercase/no-space/slugify/reverse/capitalise/csv",
            args[0]
        );
        return;
    };

    // TODO use option & result

    let output: Result<String, Box<dyn Error>> = match &args[1][..] {
        "lowercase" => match user_string() {
            Ok(input) => convert_to_lower(&input),
            Err(_) => panic!("fck"),
        },
        "uppercase" => match user_string() {
            Ok(input) => convert_to_upper(&input),
            Err(_) => panic!("fck"),
        },
        "no-space" => match user_string() {
            Ok(input) => convert_to_spaceless(&input),
            Err(_) => panic!("fck"),
        },
        "slugify" => match user_string() {
            Ok(input) => convert_to_slug(&input),
            Err(_) => panic!("fck"),
        },
        "reverse" => match user_string() {
            Ok(input) => convert_to_backwards(&input),
            Err(_) => panic!("fck"),
        },
        "capitalise" => match user_string() {
            Ok(input) => convert_to_capitalised(&input),
            Err(_) => panic!("fck"),
        },
        "csv" => todo!("trigger multiline input into table_print functionality"),
        _ => {
            eprintln!("Invalid transformation: {}", &args[1]);
            return;
        }
    };

    match output {
        Err(error) => eprintln!("{} failed with: {}", args[1], error),
        Ok(output) => println!("{} transformation successful: {}", args[1], output),
    }
}

fn user_string() -> Result<String, Box<dyn Error>> {
    println!("What text should I transform? ");
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        input = input.trim().to_string();
        Ok(input)
    } else {
        Err("Failed to read line".into())
    }
}

// i would prefer to validate this in user_string, but i need to validate inside the convert functions...
fn validate_string(input: &str) -> bool {
    !input.trim().is_empty()
}

fn convert_to_lower(input: &str) -> Result<String, Box<dyn Error>> {
    if validate_string(input) {
        let output = input.to_lowercase();
        Ok(output)
    } else {
        Err("No valid string inserted".into())
    }
}
fn convert_to_upper(input: &str) -> Result<String, Box<dyn Error>> {
    if validate_string(input) {
        let output = input.to_uppercase();
        Ok(output)
    } else {
        Err("No valid string inserted".into())
    }
}
fn convert_to_spaceless(input: &str) -> Result<String, Box<dyn Error>> {
    if validate_string(input) {
        let output = input.replace(' ', "");
        Ok(output)
    } else {
        Err("No valid string inserted".into())
    }
}
fn convert_to_backwards(input: &str) -> Result<String, Box<dyn Error>> {
    if validate_string(input) {
        let output = input.chars().rev().collect();
        Ok(output)
    } else {
        Err("No valid string inserted".into())
    }
}
fn convert_to_capitalised(input: &str) -> Result<String, Box<dyn Error>> {
    if validate_string(input) {
        //i had no idea that i've used format! it ahead of time :)
        let output = format!("{}{}", &input[..1].to_string().to_uppercase(), &input[1..]);
        Ok(output)
    } else {
        Err("No valid string inserted".into())
    }
}
fn convert_to_slug(input: &str) -> Result<String, Box<dyn Error>> {
    if validate_string(input) {
        let output = slugify(input);
        Ok(output)
    } else {
        Err("No valid string inserted".into())
    }
}
