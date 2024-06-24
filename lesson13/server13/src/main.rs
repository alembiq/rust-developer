use std::collections::HashMap;
use std::env;
use std::fs::{self};
use std::net::{SocketAddr, TcpListener, TcpStream};

// use eyre::{Context, Result};

use shared13::{
    create_directory, current_time, incoming_message, outgoing_message, server_address, MessageType,
};

static DIRECTORY_FILES: &str = "files";
static DIRECTORY_IMAGES: &str = "images";

fn main() {
    let server_address: String = server_address(env::args().collect());

    println!("{} Starting server on {}!", current_time(), server_address);
    //create directorys to store incomming objects
    create_directory(DIRECTORY_FILES);
    create_directory(DIRECTORY_IMAGES);
    listen_and_accept(server_address)
}

// Accepting communication from client and processing their messages.
fn listen_and_accept(address: String) {
    //FIXME error listening
    let listener = TcpListener::bind(address).unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr, stream.try_clone().unwrap());
        let message = incoming_message(clients.get(&addr).unwrap().try_clone().unwrap());
        //FIXME notify user connect/disconnect
        match message {
            MessageType::Text(text) => {
                //TEXT message
                println!("{} {text:?}", current_time());
            }
            MessageType::File(name, content) => {
                //FILE transfer
                //TODO unable to save
                //TODO file already exist
                println!("{} saving: {}/{}", current_time(), DIRECTORY_FILES, name);
                fs::write(format!("{}/{}", DIRECTORY_FILES, name), content)
                    .expect("Could not write file");
            }
            MessageType::Image(image) => {
                //IMAGE transfer
                //TODO unable to save
                //TODO file already exist
                let timestamp: String = std::time::UNIX_EPOCH
                    .elapsed()
                    .unwrap()
                    .as_secs()
                    .to_string();
                println!(
                    "{} saving: {}/{}.png",
                    current_time(),
                    DIRECTORY_IMAGES,
                    timestamp
                );
                fs::write(format!("{}/{}.png", DIRECTORY_IMAGES, timestamp), &image)
                    .expect("Could not write file");
            }
        }
        let response = MessageType::Text("󰸞".to_string());
        outgoing_message(&mut stream, &response);
    }
}
