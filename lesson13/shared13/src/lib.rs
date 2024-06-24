use std::fs::{self};
use std::io::{Cursor, Read, Write};
use std::net::{IpAddr, TcpStream};
use std::path::Path;
use std::process;

use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub static DEFAULT_ADDRESS: &str = "127.0.0.1:11111";

#[derive(Error, Debug)]
pub enum ErrorMessage {
    #[error("File name wasn't provided.")]
    FileNotNamed(String),
    #[error("File {0} not found.")]
    FileNotFound(String),
    #[error("Failed to read from file.")]
    FileReadFailed(#[from] std::io::Error),
    #[error("File {0} bigger than allowed size {1}.")]
    FileTooBig(String, u32),

    #[error("Unsupported image format.")]
    UnsupportedImage(#[from] image::ImageError),
}

pub fn current_time() -> String {
    std::time::UNIX_EPOCH
        .elapsed()
        .unwrap()
        .as_secs()
        .to_string()
}

/// CONNECTIVITY

pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

pub fn server_address(args: Vec<String>) -> String {
    if args.len() > 1 && args[1] == "help" {
        println!("=============== USAGE ===============");
        println!("{} IPaddress:port", args[0]);
        process::exit(1)
    } else if args.len() > 1 && args[1].parse::<IpAddr>().is_ok() {
        args[1].clone()
    } else {
        //TODO if IP is not valid notify user
        DEFAULT_ADDRESS.to_string()
    }
}


/// MESSAGE HANDLING  mayre redo in the future as implementations of MessaType

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    File(String, Vec<u8>), // Filename and its content as bytes
    Image(Vec<u8>),
    Text(String),
}

pub fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(&message).unwrap()
}

pub fn deserialize_message(input: &[u8]) -> MessageType {
    serde_json::from_slice(input).unwrap()
}

pub fn incoming_message(stream: &mut TcpStream) -> MessageType {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).unwrap();
    deserialize_message(&buffer)
}

pub fn outgoing_message(stream: &mut TcpStream, message: &MessageType) {
    let serialized = serialize_message(message);
    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes()).unwrap();
    stream.write_all(serialized.as_bytes()).unwrap();
}

/// FILE HANDLING

pub fn read_file(input: String) -> Vec<u8> {
    let mut filename = input.split_whitespace();
    let filename: &str = filename.nth(1).expect("missing filename");
    //FIXME better error
    std::fs::read(format!("./{}", filename)).unwrap()
}

pub fn create_directory(directory: &str) {
    //TODO check for directory existence

    if !Path::new(directory).is_dir() {
        fs::create_dir(directory).unwrap();
        println!("{} creating {} directory", current_time(), { directory });
    }
}
// pub fn filename_from_input(user_input: &str) -> Result<&str, ErrorMessage> {
//     let filename = user_input.split(' ').nth(1).expect("missing filename");
//     if filename.is_empty() {
//         return  Err(ErrorMessage::FileNotFound(filename.to_string()));
//     }
//     Ok(filename)
// }

pub fn filename_from_input(user_input: &str) -> &str {
    user_input.split(' ').nth(1).expect("missing filename")
}

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
    //TODO encoding failed
    output.into_inner() as Vec<u8>
}
