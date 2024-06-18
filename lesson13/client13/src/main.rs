use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use shared13::{
    current_time, incoming_message, outgoing_message, read_file, server_address, MessageType,
};
use std::env;
use std::io::{self, Cursor};
use std::net::TcpStream;

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
        let user_input = user_input.trim();

        let message: MessageType = {
            //Scan user input for commands
            if user_input.starts_with(".quit") {
                //QUIT client
                println!("{} Exiting!", current_time(),);
                break;
            } else if user_input.starts_with(".file") {
                //send FILE
                let mut file = user_input.split(' ');
                let filename: &str = file.nth(1).expect("missing filename");
                MessageType::File(filename.to_string(), read_file(user_input.to_string()))
            } else if user_input.starts_with(".image") {
                //send file as PNG
                let mut file = user_input.split(' ');
                let filename: &str = file.nth(1).expect("missing filename");
                let img = image::open(filename).unwrap();
                let mut output = Cursor::new(Vec::new());
                let encoder = PngEncoder::new(&mut output);
                let _ = encoder.write_image(
                    img.as_bytes(),
                    img.width(),
                    img.height(),
                    img.color().into(),
                );
                MessageType::Image(output.into_inner() as Vec<u8>)
            } else {
                //no command, just TEXT
                MessageType::Text(user_input.to_string())
            }
        };

        let mut stream = TcpStream::connect(&address).unwrap();
        outgoing_message(&mut stream, &message);
        let response = incoming_message(stream);
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
