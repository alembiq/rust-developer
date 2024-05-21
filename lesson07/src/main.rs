use slug::slugify;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::process;
use std::sync::mpsc;
use std::thread;

enum Operations {
    Lowercase,
    Uppercase,
    NoSpace,
    Slugify,
    Reverse,
    Capitalise,
    Csv,
}
impl Operations {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "lowercase" => Ok(Operations::Lowercase),
            "uppercase" => Ok(Operations::Uppercase),
            "no-spaces" => Ok(Operations::NoSpace),
            "slugify" => Ok(Operations::Slugify),
            "reverse" => Ok(Operations::Reverse),
            "capitalise" => Ok(Operations::Capitalise),
            "csv" => Ok(Operations::Csv),
            _ => Err(format!("{} is not a valid transformation", s)),
        }
    }
}
impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            Operations::Lowercase => "lowercase",
            Operations::Uppercase => "uppercase",
            Operations::NoSpace => "no-space",
            Operations::Slugify => "slugify",
            Operations::Reverse => "reverse",
            Operations::Capitalise => "capitalise",
            Operations::Csv => "csv",
        };
        write!(f, "{}", variant_str)
    }
}

fn main() {
    let ask_for_tranformation = String::from("What transformation do you need (lowercase/uppercase/no-space/slugify/reverse/capitalise/csv)?");
    let (tx, rx) = mpsc::channel(); // mpsc::channel::() mpsc::channel::()() mpsc::channel::(T)() mpsc::channel::<T>()
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Let's go interactive!");
        let input_thread = thread::Builder::new().name("input_thread".into());
        let output_thread = thread::Builder::new().name("output_thread".into());

        let input_handle = input_thread
            .spawn(move || loop {
                match user_input(&ask_for_tranformation) {
                    Ok(transformation) => {
                        match Operations::from_str(&transformation) {
                            Ok(variant) => {
                                let text = user_input(&variant.to_string());
                                tx.send(transformation).unwrap();
                                tx.send((text).expect("REASON")).unwrap();
                                thread::sleep(std::time::Duration::from_secs(1));
                            }
                            Err(err) => {
                                eprintln!("{}, exiting", err);
                                process::exit(1);
                            }
                        };
                    }
                    Err(error) => eprintln!("{}", error),
                };
            })
            .unwrap();

        let output_handle = output_thread
            .spawn(move || {
                let mut operation = String::new();
                let mut text = String::new();
                loop {
                    if operation.len() > 1 && text.len() > 1 {
                        println!("{} {}", operation, text);
                        let output: Result<String, Box<dyn Error>> = match &*operation {
                            "lowercase" => convert_to_lower(&text),
                            "uppercase" => convert_to_upper(&text),
                            "no-space" => convert_to_spaceless(&text),
                            "slugify" => convert_to_slug(&text),
                            "reverse" => convert_to_backwards(&text),
                            "capitalise" => convert_to_capitalised(&text),
                            "csv" => print_table(user_csv(&text)),
                            &_ => {
                                eprintln!("Invalid transformation: {}", operation);
                                return;
                            }
                        };
                        match output {
                            Err(error) => eprintln!("{} failed with: {}", operation, error),
                            Ok(output) => {
                                println!("{} transformation successful:\n{}", operation, output)
                            }
                        }
                        operation = "".to_string();
                        text = "".to_string();
                    } else if operation.is_empty() {
                        match rx.recv() {
                            Err(err) => {
                                eprintln!("OUTPUT[ERROR]: {}", err);
                            }
                            Ok(input) => {
                                operation = input;
                            }
                        };
                    } else {
                        match rx.recv() {
                            Err(err) => {
                                eprintln!("OUTPUT[ERROR]: {}", err);
                            }
                            Ok(input) => {
                                text = input;
                            }
                        };
                    }
                }
            })
            .unwrap();

        input_handle.join().unwrap();
        output_handle.join().unwrap();
    } else {
        let output: Result<String, Box<dyn Error>> = match &args[1][..] {
            "lowercase" => match user_input(&args[1]) {
                Ok(input) => convert_to_lower(&input),
                Err(_) => panic!("fck"),
            },
            "uppercase" => match user_input(&args[1]) {
                Ok(input) => convert_to_upper(&input),
                Err(_) => panic!("fck"),
            },
            "no-space" => match user_input(&args[1]) {
                Ok(input) => convert_to_spaceless(&input),
                Err(_) => panic!("fck"),
            },
            "slugify" => match user_input(&args[1]) {
                Ok(input) => convert_to_slug(&input),
                Err(_) => panic!("fck"),
            },
            "reverse" => match user_input(&args[1]) {
                Ok(input) => convert_to_backwards(&input),
                Err(_) => panic!("fck"),
            },
            "capitalise" => match user_input(&args[1]) {
                Ok(input) => convert_to_capitalised(&input),
                Err(_) => panic!("fck"),
            },
            "csv" => match user_input(&args[1]) {
                Ok(input) => print_table(user_csv(&input)),
                Err(_) => panic!("fck"),
            },
            _ => {
                println!(
                    "Usage: {} lowercase/uppercase/no-space/slugify/reverse/capitalise/csv",
                    args[0]
                );
                eprintln!("Invalid transformation: {}", &args[1]);
                return;
            }
        };
        match output {
            Err(error) => eprintln!("{} failed with: {}", args[1], error),
            Ok(output) => println!("{} transformation successful:\n{}", args[1], output),
        }
    }
}

fn user_input(operation: &String) -> Result<String, Box<dyn Error>> {
    if operation == "csv" {
        println!("What CSV file should I display? ");
    } else {
        println!("What text should I {}? ", operation);
    }

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        input = input.trim().to_string();
        Ok(input)
    } else {
        Err("Failed to read line".into())
    }
}
fn user_csv(file_name: &String) -> std::option::Option<String> {
    let mut content = String::new();
    let mut file = File::open(file_name).expect("Unable to open the file:");
    file.read_to_string(&mut content)
        .expect("Unable to read the file");

    Some(content)
}

fn print_table(input: std::option::Option<String>) -> Result<String, Box<dyn Error>> {
    if validate_csv(&input) {
        let table = csv_to_table::from_reader(input.expect("REASON").as_bytes()).unwrap();
        Ok(table.to_string())
    } else {
        Err("CSV invalid".into())
    }
}

fn validate_string(input: &str) -> bool {
    !input.trim().is_empty()
}

fn validate_csv(input: &Option<String>) -> bool {
    let binding = <Option<String> as Clone>::clone(input).unwrap();
    let mut rdr = csv::Reader::from_reader(binding.as_bytes());
    if let Some(result) = rdr.records().next() {
        match result {
            Err(_) => return false,
            Ok(_) => return true,
        }
    }
    true
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
