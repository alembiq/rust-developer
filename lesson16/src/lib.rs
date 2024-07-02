#![warn(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/alembiq/rust-developer/issues/")]
//! Just a simple CLI chat
//!
//! Able to send messages, files and images (always converted to png)
//! created for a `RUST` training course with [robot_dreams](https://robot-dreams-rust.mag.wiki/)

use std::error::Error;
use std::fs::{self};
use std::io::{Cursor, Read, Write};
use std::net::{IpAddr, TcpStream};
use std::path::Path;
use std::process;

use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Default server  127.0.0.1:11111
pub static DEFAULT_ADDRESS: &str = "127.0.0.1:11111";
/// Default folder for storing incoming `files`
pub static DIRECTORY_FILES: &str = "files";
/// Default folder for storing incoming `images`
pub static DIRECTORY_IMAGES: &str = "images";

#[doc(hidden)]
/// Error messages used by [thiserror](https://docs.rs/thiserror/latest/thiserror/)
#[derive(Error, Debug)]
pub enum ErrorMessage {
    #[error("File {0} not found.")]
    FileNotFound(String),
    #[error("File {0} bigger than allowed size {1}.")]
    FileTooBig(String, u32),

    #[error("Unsupported image format.")]
    ImageUnsupported(#[from] image::ImageError),

    #[error("Failed to read message or file")]
    IoError(#[from] std::io::Error),
    #[error("Message has invalid format")]
    InvalidMessageFormat(#[from] ciborium::de::Error<std::io::Error>),
}

/// Returns timestamp as String
pub fn timestamp() -> String {
    std::time::UNIX_EPOCH
        .elapsed()
        .unwrap()
        .as_secs()
        .to_string()
}

/// Return server address
/// eighter one from the CLI parameter or a default one
///
/// # Example
/// ```
/// # use lesson16::server_address;
/// # use std::env;
/// let server_address: String = server_address(env::args().collect());
/// println!("{}",server_address);
/// ```
pub fn server_address(args: Vec<String>) -> String {
    if args.len() > 1 && args[1] == "help" {
        println!("=============== USAGE ===============");
        println!("{} IPaddress:port", args[0]);
        process::exit(0)
    } else if args.len() > 1 {
        let ip: Vec<&str> = args[1].split(':').collect();
        assert!(ip[0].parse::<IpAddr>().is_ok());
        assert!((1..65535).contains(&ip[1].parse::<i32>().unwrap()));
        args[1].clone()
    } else {
        DEFAULT_ADDRESS.to_string()
    }
}

/// Define types of messages
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    /// File as name and content.
    File(String, Vec<u8>),
    /// 'PNG' image as content.
    Image(Vec<u8>),
    /// Text.
    Text(String),
}

/// Process incomming [MessageType]
pub fn incoming_message(stream: &mut TcpStream) -> Result<MessageType, ErrorMessage> {
    let mut len_bytes = [0; 4];
    stream.read_exact(&mut len_bytes)?;
    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0; len];
    stream.read_exact(&mut buffer)?;

    Ok(ciborium::from_reader(&mut &buffer[..])?)
}

/// Send [MessageType]
pub fn outgoing_message(
    stream: &mut TcpStream,
    message: &MessageType,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::new();
    ciborium::into_writer(message, &mut buffer)?;
    let len = buffer.len() as u32;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&buffer)?;
    Ok(())
}

/// Reads file to be send
pub fn read_file(input: String) -> Vec<u8> {
    let mut filename = input.split_whitespace();
    let filename: &str = filename.nth(1).expect("missing filename");
    std::fs::read(format!("./{}", filename)).unwrap()
}

/// Creates directory for files/images to be stored
pub fn create_directory(directory: &str) {
    if !Path::new(directory).is_dir() {
        fs::create_dir(directory).unwrap();
        println!("{} creating {} directory", timestamp(), { directory });
    }
}

/// Reads filename from a user command
pub fn filename_from_input(user_input: &str) -> Result<&str, ErrorMessage> {
    let filename = user_input.split(' ').nth(1).expect("missing filename");
    if filename.is_empty() {
        return Err(ErrorMessage::FileNotFound(filename.to_string()));
    }
    Ok(filename)
}

/// Converts image to `PNG`
/// - meant to be used on sender side
pub fn image_to_png(file: &str) -> Vec<u8> {
    let img = image::open(file).unwrap();
    let mut output = Cursor::new(Vec::new());
    let encoder = PngEncoder::new(&mut output);
    let _ = encoder.write_image(
        img.as_bytes(),
        img.width(),
        img.height(),
        img.color().into(),
    );
    output.into_inner() as Vec<u8>
}
