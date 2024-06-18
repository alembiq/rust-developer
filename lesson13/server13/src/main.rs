use shared13::{
    create_folder, current_time, incoming_message, outgoing_message, server_address, MessageType,
};
use std::collections::HashMap;
use std::env;
use std::fs::{self};
use std::net::{SocketAddr, TcpListener, TcpStream};

static FOLDER_FILES: &str = "files";
static FOLDER_IMAGES: &str = "images";

fn main() {
    let server_address: String = server_address(env::args().collect());

    println!("{} Starting server on {}!", current_time(), server_address);
    //create folders to store incomming objects
    create_folder(FOLDER_FILES);
    create_folder(FOLDER_IMAGES);
    listen_and_accept(server_address)
}

// Accepting communication from client and processing their messages.
fn listen_and_accept(address: String) {
    let listener = TcpListener::bind(address).unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr, stream.try_clone().unwrap());
        let message = incoming_message(clients.get(&addr).unwrap().try_clone().unwrap());

        match message {
            MessageType::Text(text) => {
                //TEXT message
                println!("{} {text:?}", current_time());
            }
            MessageType::File(name, content) => {
                //FILE transfer
                println!("{} saving: {}/{}", current_time(), FOLDER_FILES, name);
                fs::write(format!("{}/{}", FOLDER_FILES, name), content)
                    .expect("Could not write file");
            }
            MessageType::Image(image) => {
                //IMAGE transfer
                let timestamp: String = std::time::UNIX_EPOCH
                    .elapsed()
                    .unwrap()
                    .as_secs()
                    .to_string();
                println!(
                    "{} saving: {}/{}.png",
                    current_time(),
                    FOLDER_IMAGES,
                    timestamp
                );
                fs::write(format!("{}/{}.png", FOLDER_IMAGES, timestamp), &image)
                    .expect("Could not write file");
            }
        }
        let response = MessageType::Text("󰸞".to_string());
        outgoing_message(&mut stream, &response);
    }
}
