use std::env;
use std::io;
use std::net::TcpStream;
use std::process;
use std::thread::{self, JoinHandle};

use eyre::{anyhow, bail, Context, Error, Result};

use shared13::{
    current_time, filename_from_input, image_to_png, incoming_message, outgoing_message, read_file,
    server_address, MessageType,
};

fn main() -> Result<()> {
    let server_address: String = server_address(env::args().collect());

    println!(
        "{} Client connecting to {}!",
        current_time(),
        server_address
    );

    //TODO error connecting
    //FIXME while outside loop, only one message is send
    let outgoing_stream = match TcpStream::connect(server_address) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Unable to connect: {e}");
            process::exit(1)
        }
    };

    let incomming_stream = outgoing_stream
        .try_clone()
        .context("TCP stream was already closed")?;

    let outgoing_handle = outgoing(outgoing_stream);
    let incomming_handle = incomming(incomming_stream);

    outgoing_handle?
        .join()
        .map_err(|_| anyhow!("Thread panicked"))
        .context("Failed to spawn outgoing thread")?
        .context("Outgoing handle returned error")?;

    incomming_handle
        .join()
        .map_err(|_| anyhow!("Failed to spawn incoming thread, or thread panicked"))?;

    Ok(())
}

fn outgoing(mut stream: TcpStream) -> Result<JoinHandle<Result<()>>> {
    let help_message = "What to send? (text / .image <filename> / .file <filename> / .quit): ";

    println!("{help_message}");

    let handle = thread::spawn(move || loop {
        let mut user_input: String = Default::default();
        io::stdin().read_line(&mut user_input)?;

        //FIXME better error
        let trimmed_input = user_input.trim();

        let message: MessageType = {
            match user_input.split_whitespace().next().unwrap_or_default() {
                ".quit" => {
                    println!("{} Exiting!", current_time(),);
                    process::exit(0)
                }
                ".file" => {
                    //TODO cannot read file
                    //TODO file size check
                    //TODO create function for reading file
                    MessageType::File(
                        filename_from_input(trimmed_input)?.to_string(),
                        read_file(trimmed_input.to_string()),
                    )
                }
                ".image" => {
                    //TODO cannot read file
                    //TODO file isn't image
                    //TODO file size check
                    MessageType::Image(image_to_png(filename_from_input(trimmed_input)?))
                }
                _ => {
                    //TODO handle empty string ? write help
                    if trimmed_input.is_empty() {
                        println!("{help_message}");
                    }

                    MessageType::Text(trimmed_input.to_string())
                }
            }
        };

        outgoing_message(&mut stream, &message);
    });

    Ok(handle)
}

fn incomming(mut stream: TcpStream) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let response = match incoming_message(&mut stream) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("received message was invalid: {e}");
                continue;
            }
        };

        println!(
            "{} server: {}",
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
    })
}
