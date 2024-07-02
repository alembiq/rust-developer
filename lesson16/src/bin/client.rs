use std::env;
use std::fs;
use std::io;
use std::net::TcpStream;
use std::process;
use std::thread::{self, JoinHandle};

use eyre::{anyhow, bail, Context, Result};

use lesson16::{
    create_directory, filename_from_input, image_to_png, incoming_message, outgoing_message,
    read_file, server_address, timestamp, MessageType, DIRECTORY_FILES, DIRECTORY_IMAGES,
};

fn main() -> Result<()> {
    let server_address: String = server_address(env::args().collect());

    println!("{} Client connecting to {}!", timestamp(), server_address);

    let outgoing_stream = match TcpStream::connect(server_address) {
        Ok(s) => s,
        Err(e) => {
            bail!("Unable to connect: {e}")
        }
    };

    let incoming_stream = outgoing_stream
        .try_clone()
        .context("TCP stream was already closed")?;

    let outgoing_handle = outgoing(outgoing_stream);
    let incoming_handle = incoming(incoming_stream);

    outgoing_handle?
        .join()
        .map_err(|_| anyhow!("Thread panicked"))
        .context("Failed to spawn outgoing thread")?
        .context("Outgoing handle returned error")?;

    incoming_handle
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
        let trimmed_input = user_input.trim();
        if trimmed_input.is_empty() {
            println!("{help_message}");
        } else {
            let message: MessageType = {
                match trimmed_input.split_whitespace().next().unwrap_or_default() {
                    ".quit" => {
                        println!("{} Exiting!", timestamp(),);
                        process::exit(0)
                    }
                    ".file" => MessageType::File(
                        filename_from_input(trimmed_input)?.to_string(),
                        read_file(trimmed_input.to_string()),
                    ),
                    ".image" => {
                        MessageType::Image(image_to_png(filename_from_input(trimmed_input)?))
                    }
                    _ => MessageType::Text(trimmed_input.to_string()),
                }
            };
            if let Err(e) = outgoing_message(&mut stream, &message) {
                eprintln!(
                    "{} Failed to broadcast message: {message:?} -> {e}",
                    timestamp()
                );
            }
        };
    });

    Ok(handle)
}

fn incoming(mut stream: TcpStream) -> JoinHandle<()> {
    create_directory(DIRECTORY_FILES);
    create_directory(DIRECTORY_IMAGES);
    thread::spawn(move || loop {
        let message = match incoming_message(&mut stream) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("{} Stream inter: {e}", timestamp());
                //FIXME infinite loop
                process::exit(1);
            }
        };

        match message {
            MessageType::Text(text) => {
                println!("{} {text:?}", timestamp());
            }
            MessageType::File(name, content) => {
                //TODO unable to save
                //TODO file already exist
                fs::write(format!("{}/{}", DIRECTORY_FILES, name), content)
                    .expect("Could not write file");
                println!("{} Receiving {name}", timestamp());
            }
            MessageType::Image(image) => {
                //TODO unable to save
                //TODO file already exist
                let filename: String = std::time::UNIX_EPOCH
                    .elapsed()
                    .unwrap()
                    .as_secs()
                    .to_string();
                fs::write(format!("{}/{}.png", DIRECTORY_IMAGES, filename), &image)
                    .expect("Could not write file");
                println!("{} Receiving {filename}.png", timestamp());
            }
        }
    })
}
