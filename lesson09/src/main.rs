use lesson09::{deserialize_message, serialize_message, MessageType};
use std::collections::HashMap;
use std::env;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

fn main() {
    let default_address = "127.0.0.1:11111";

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "help" {
        println!("=============== USAGE ===============");
        println!("{} server listenerIP:port", args[0]);
        println!("{} client serverIP:port", args[0]);
    } else if args.len() > 1 && args[1] == "server" {
        server(if args.len() > 2 && is_valid_ip(&args[2]) {
            &args[2]
        } else {
            default_address
        });
    } else {
        client(if args.len() > 2 && is_valid_ip(&args[2]) {
            &args[2]
        } else {
            default_address
        });
    }
}

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

fn server(address: &str) {
    println!(
        "{} Starting server!",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    );
    listen_and_accept(address)
    /*
    Display a notification like Receiving image... or Receiving <filename> for incoming files.
    .image -> images/<timestamp>.png (convert from other formats)
    .file -> files/originalfilename
    incoming text messages, display them directly in stdout.
    */
}

fn client(address: &str) {
    println!(
        "{} Starting client!",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    );
    /*
    .file <path>: Sends a file to the server.
    .image <path>: Sends an image (assumed or required to be .png).
    .quit command should terminate the client.
    Any other text: Considered a standard text message.
    */
    let new_message = MessageType::Text("Hello from client!".to_string());
    let mut stream = TcpStream::connect(address).unwrap();
    send_message(&mut stream, &new_message);

    let response = handle_client(stream);
    println!("{response:?}");
}

fn send_message(stream: &mut TcpStream, message: &MessageType) {
    let serialized = serialize_message(message);
    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes()).unwrap();
    stream.write_all(serialized.as_bytes()).unwrap();
}

fn listen_and_accept(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr, stream.try_clone().unwrap());
        let message = handle_client(clients.get(&addr).unwrap().try_clone().unwrap());
        // Here, you can further process this message as per your requirements
        println!(
            "{} {message:?}",
            std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
        );
        let response = MessageType::Text("Received".to_string());
        send_message(&mut stream, &response);
    }
}

fn handle_client(mut stream: TcpStream) -> MessageType {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).unwrap();
    deserialize_message(&buffer)
}
