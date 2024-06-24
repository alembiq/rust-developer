use std::env;
use std::io::{self};
use std::net::TcpStream;

// use eyre::{Context, Error};

use shared13::{
    current_time, filename_from_input, image_to_png, incoming_message, outgoing_message, read_file,
    server_address, MessageType,
};

fn main() {
    let server_address: String = server_address(env::args().collect());

    println!(
        "{} Client connecting to {}!",
        current_time(),
        server_address
    );
    user_actions(server_address);
}

fn user_actions(address: String) {
    loop {
        println!(
            "{} What to send? (text / .image <filename> / .file <filename> / .quit): ",
            current_time(),
        );

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        //FIXME better error
        let user_input = user_input.trim();

        let message: MessageType = {
            //Scan user input for commands
            if user_input.starts_with(".quit") {
                //QUIT client
                println!("{} Exiting!", current_time(),);
                break;
            } else if user_input.starts_with(".file") {
                //send FILE
                //TODO cannot read file
                //TODO file size check
                //TODO create function for reading file
                MessageType::File(
                    filename_from_input(user_input).to_string(),
                    read_file(user_input.to_string()),
                )
            } else if user_input.starts_with(".image") {
                //send file as PNG
                //TODO cannot read file
                //TODO file isn't image
                //TODO file size check
                MessageType::Image(image_to_png(filename_from_input(user_input)))
            } else {
                //no command, just TEXT
                MessageType::Text(user_input.to_string())
            }
        };

        let mut stream = TcpStream::connect(&address).unwrap();
        //TODO error connecting
        outgoing_message(&mut stream, &message);
        let response = incoming_message(stream);
        //TODO undelivered
        println!(
            "{} server response: {}",
            current_time(),
            match response {
                MessageType::Text(text) => {
                    //TEXT message
                    text
                }
                MessageType::Image(_text) => {
                    //TEXT message
                    "image".to_string()
                }
                MessageType::File(_text, _content) => {
                    //TEXT message
                    "file".to_string()
                }
            }
        );
    }
}
