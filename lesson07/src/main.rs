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

//TODO create separate file for enum Operator
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
    let ask_for_string = String::from("What text should I transform?");
    let ask_for_file = String::from("What CSV file should I process?");
    let (tx, rx) = mpsc::channel(); //FIXME WTF uz me to reklo ze doplnit T, druhy zavorky, nahradit zavorky ostrejma a pak umrelo

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("let's go interactive");

        let input_thread = thread::Builder::new().name("input_thread".into());
        let output_thread = thread::Builder::new().name("output_thread".into());

        let input_handle = input_thread
            .spawn(move || loop {
                //TODO get Operation - valid or die
                //TODO get Text or content of a File
                match user_input(&ask_for_tranformation) {
                    Ok(transformation) => {
                        match Operations::from_str(&transformation) {
                            Ok(variant) => {
                                //TODO adjust user_input to return text from input or file based on Operation
                                let text = user_input(&variant.to_string());
                                tx.send(transformation).unwrap();
                                tx.send((text).expect("REASON")).unwrap();
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

        //     let input = user_input(&ask_for_tranformation);
        //     match &input {
        //         Ok(transformation) => {
        //             tx.send(input).unwrap()
        //             // match transformation {
        //             //     Operations => tx.send(transformation).unwrap(), // Send a message through the channel
        //             // }
        //         },
        //         Err(_) => panic!("some error"),
        //     }
        let output_handle = output_thread
            .spawn(move || loop {
                //TODO evaluate Operation and execute on Text
                //TODO println! output
                // for request in rx {
                // println!("{}", request);
                //         // let (command, input_str): (_, _) = request;
                //         // match run(command, input_str) {
                //         //     Err(e) => eprintln!("Procesing Error: {}", e),
                //         //     Ok(transmuted_str) => println!("Transmutation result: \n{}", transmuted_str),
                //         // };
                // }
                match rx.recv() {
                    Err(err) => {
                        eprintln!("OUTPUT[ERROR]: {}",err);
                    }
                    Ok(msg) => {
                        println!("OUTPUT: {}", msg);
                    }
                };
                // }
            })
            .unwrap();

        input_handle.join().unwrap();
        output_handle.join().unwrap();

    //     loop {
    //         let output = match user_input(&ask_for_tranformation) {
    //             Ok(transformation) => match &*transformation {
    //                 "lowercase" => match user_input(&ask_for_string) {
    //                     Ok(input) => convert_to_lower(&input),
    //                     Err(_) => panic!("fck"),
    //                 },
    //                 "uppercase" => match user_input(&ask_for_string) {
    //                     Ok(input) => convert_to_upper(&input),
    //                     Err(_) => panic!("fck"),
    //                 },
    //                 "no-space" => match user_input(&ask_for_string) {
    //                     Ok(input) => convert_to_spaceless(&input),
    //                     Err(_) => panic!("fck"),
    //                 },
    //                 "slugify" => match user_input(&ask_for_string) {
    //                     Ok(input) => convert_to_slug(&input),
    //                     Err(_) => panic!("fck"),
    //                 },
    //                 "reverse" => match user_input(&ask_for_string) {
    //                     Ok(input) => convert_to_backwards(&input),
    //                     Err(_) => panic!("fck"),
    //                 },
    //                 "capitalise" => match user_input(&ask_for_string) {
    //                     Ok(input) => convert_to_capitalised(&input),
    //                     Err(_) => panic!("fck"),
    //                 },
    //                 "csv" => match user_input(&ask_for_file) {
    //                     Ok(input) => print_table(user_csv(&input)),
    //                     Err(_) => panic!("fck"),
    //                 },
    //                 "" => {
    //                     println!("No transformation requested, quiting");
    //                     break;
    //                 }
    //                 _ => {
    //                     println!("Invalid transformation: {}", &transformation);
    //                     return;
    //                 }
    //             },
    //             Err(_) => panic!("some error"),
    //         };
    //         match output {
    //             Err(error) => eprintln!("{} failed with", error),
    //             Ok(output) => println!("transformation successful:\n\n{}\n", output),
    //         }
    //     }
    //     return;
    } else {
        let output: Result<String, Box<dyn Error>> = match &args[1][..] {
            "lowercase" => match user_input(&ask_for_string) {
                Ok(input) => convert_to_lower(&input),
                Err(_) => panic!("fck"),
            },
            "uppercase" => match user_input(&ask_for_string) {
                Ok(input) => convert_to_upper(&input),
                Err(_) => panic!("fck"),
            },
            "no-space" => match user_input(&ask_for_string) {
                Ok(input) => convert_to_spaceless(&input),
                Err(_) => panic!("fck"),
            },
            "slugify" => match user_input(&ask_for_string) {
                Ok(input) => convert_to_slug(&input),
                Err(_) => panic!("fck"),
            },
            "reverse" => match user_input(&ask_for_string) {
                Ok(input) => convert_to_backwards(&input),
                Err(_) => panic!("fck"),
            },
            "capitalise" => match user_input(&ask_for_string) {
                Ok(input) => convert_to_capitalised(&input),
                Err(_) => panic!("fck"),
            },
            "csv" => match user_input(&ask_for_file) {
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

fn user_input(question: &String) -> Result<String, Box<dyn Error>> {
    println!("{} ", question);
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

// i would prefer to validate this in user_input, but i need to validate inside the convert functions...
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
