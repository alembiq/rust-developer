// use anyhow::{Context, Result}; //FIXME remove
// use bincode;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;

use std::error::Error;
use std::fs::{self};
use std::io::{Cursor, Read, Write};
use std::marker::Unpin;
use std::net::{IpAddr, TcpStream as TcpSync};
use std::path::Path;
use std::process;

pub static DEFAULT_ADDRESS: &str = "127.0.0.1:11111";
pub static DIRECTORY_FILES: &str = "files";
pub static DIRECTORY_IMAGES: &str = "images";

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

pub fn timestamp() -> String {
    std::time::UNIX_EPOCH
        .elapsed()
        .unwrap()
        .as_secs()
        .to_string()
}

/// CONNECTIVITY

pub fn server_address(args: Vec<String>) -> String {
    let ip: Vec<&str> = args[1].split(':').collect();

    if args.len() > 1 && args[1] == "help" {
        println!("=============== USAGE ===============");
        println!("{} IPaddress:port", args[0]);
        process::exit(0)
    } else if args.len() > 1 {
        assert!(ip[0].parse::<IpAddr>().is_ok());
        assert!((1..65535).contains(&ip[1].parse::<i32>().unwrap()));
        args[1].clone()
    } else {
        DEFAULT_ADDRESS.to_string()
    }
}

/// MESSAGE HANDLING  maybe redo as implementations of MessageType

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    File(String, Vec<u8>), // Filename and its content as bytes
    Image(Vec<u8>),
    Text(String),
}

impl MessageType {
    pub async fn recv<T: AsyncReadExt + Unpin>(stream: &mut T) -> Result<Self, ErrorMessage> {
        let mut length_bytes = [0; 4];

        stream
            .read_exact(&mut length_bytes)
            .await
            .expect("{timestamp()} Failed to read length");

        let length = u32::from_be_bytes(length_bytes);

        let mut buf = vec![0; length as usize];

        stream
            .read_exact(&mut buf)
            .await
            .expect("{timestamp()} Failed to read message");

        Ok(bincode::deserialize(&buf).expect("{timestamp()} Unable to deserialize message."))
    }

    pub async fn send<T: AsyncWriteExt + Unpin + AsyncReadExt>(
        &self,
        stream: &mut T,
    ) -> Result<(), ErrorMessage> {
        let serialized: Vec<u8> =
            bincode::serialize(&self).expect("{timestamp()} Failed to serialize message");

        let length = (serialized.len() as u32).to_be_bytes();

        stream
            .write_all(&length)
            .await
            .expect("{timestamp()} Failed to send length");
        stream
            .write_all(&serialized)
            .await
            .expect("{timestamp()} Failed to send message");
        Ok(())
    }
}

pub fn message_incoming(stream: &mut TcpSync) -> Result<MessageType, ErrorMessage> {
    let mut len_bytes = [0; 4];
    stream.read_exact(&mut len_bytes)?;
    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0; len];
    stream.read_exact(&mut buffer)?;

    Ok(ciborium::from_reader(&mut &buffer[..])?)
}

pub fn message_outgoing(stream: &mut TcpSync, message: &MessageType) -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::new();
    ciborium::into_writer(message, &mut buffer)?;
    let len = buffer.len() as u32;
    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&buffer)?;
    Ok(())
}

/// FILE HANDLING

#[tokio::main]
pub async fn async_file_read(input: &str) -> Vec<u8> {
    let mut filename = input.split_whitespace();
    let filename: &str = filename.nth(1).expect("missing filename");
    let mut file = tokio::fs::File::open(format!("./{}", filename))
        .await
        .unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.unwrap();
    contents
}

#[tokio::main]
pub async fn async_file_write(filename: String, data: Vec<u8>) {
    let mut file = tokio::fs::File::create(format!("./{}", filename))
        .await
        .unwrap();
    file.write_all(data.as_ref()).await.unwrap();
}

pub fn directory_create(directory: &str) {
    if !Path::new(directory).is_dir() {
        fs::create_dir(directory).unwrap();
        println!("{} creating {} directory", timestamp(), { directory });
    }
}

pub fn filename_from_input(user_input: &str) -> Result<&str, ErrorMessage> {
    let filename = user_input.split(' ').nth(1).expect("missing filename");
    if filename.is_empty() {
        return Err(ErrorMessage::FileNotFound(filename.to_string()));
    }
    Ok(filename)
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
    output.into_inner() as Vec<u8>
}
