use eyre::{anyhow, bail, Context, Result};

use lesson15::{
    async_file_read, async_file_write, directory_create, filename_from_input, image_to_png,
    message_incoming, message_outgoing, server_address, timestamp, MessageType, DIRECTORY_FILES,
    DIRECTORY_IMAGES,
};

use std::env;
use std::fs;
use std::io;
use std::net::TcpStream;
use std::process;
use std::thread::{self, JoinHandle};

#[tokio::main]
async fn main() -> Result<()> {
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
    let incoming_async = tokio::spawn(async {
        read_from_stream(incoming_stream);
    });
    incoming_async.await.unwrap();

    let outgoing_handle = send_to_stream(outgoing_stream);
    outgoing_handle?
        .join()
        .map_err(|_| anyhow!("Thread panicked"))
        .context("Failed to spawn outgoing thread")?
        .context("Outgoing handle returned error")?;

    Ok(())
}

fn send_to_stream(mut stream: TcpStream) -> Result<JoinHandle<Result<()>>> {
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
                        async_file_read(trimmed_input),
                    ),
                    ".image" => {
                        MessageType::Image(image_to_png(filename_from_input(trimmed_input)?))
                    }
                    _ => MessageType::Text(trimmed_input.to_string()),
                }
            };
            if let Err(e) = message_outgoing(&mut stream, &message) {
                eprintln!(
                    "{} Failed to broadcast message: {message:?} -> {e}",
                    timestamp()
                );
            }
        };
    });

    Ok(handle)
}

fn read_from_stream(mut stream: TcpStream) -> JoinHandle<()> {
    directory_create(DIRECTORY_FILES);
    directory_create(DIRECTORY_IMAGES);
    thread::spawn(move || loop {
        let message = match message_incoming(&mut stream) {
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
                async_file_write(format!("{}/{}", DIRECTORY_FILES, name), content);
            }
            MessageType::Image(image) => {
                let image_file: String = std::time::UNIX_EPOCH
                    .elapsed()
                    .unwrap()
                    .as_secs()
                    .to_string();
                fs::write(format!("{}/{}.png", DIRECTORY_IMAGES, image_file), &image)
                    .expect("Could not write file");
                println!("{} Receiving {image_file}.png", timestamp());
            }
        }
    })
}
