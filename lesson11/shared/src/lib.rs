use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
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
