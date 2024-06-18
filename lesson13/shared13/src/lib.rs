use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};

pub static DEFAULT_ADDRESS: &str = "127.0.0.1:11111";

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

pub fn incoming_message(mut stream: TcpStream) -> MessageType {
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

pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

pub fn server_address(args: Vec<String>) -> String {
    if args.len() > 1 && args[1] == "help" {
        println!("=============== USAGE ===============");
        println!("{} IPaddress:port", args[0]);
        panic!()
    } else if args.len() > 1 && args[1].parse::<IpAddr>().is_ok() {
        args[1].clone()
    } else {
        DEFAULT_ADDRESS.to_string()
    }
}

pub fn current_time() -> String {
    std::time::UNIX_EPOCH
        .elapsed()
        .unwrap()
        .as_secs()
        .to_string()
}

pub fn read_file(input: String) -> Vec<u8> {
    let mut filename = input.split(' ');
    let filename: &str = filename.nth(1).expect("missing filename");
    std::fs::read(format!("./{}", filename)).unwrap()
}
pub fn create_folder(folder: &str) {
    if let Err(why) = fs::create_dir(folder) {
        println!(
            "{} creating {} folder: {:?}",
            current_time(),
            { folder },
            why.kind()
        )
    }
}
