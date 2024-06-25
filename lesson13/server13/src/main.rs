use std::collections::HashMap;
use std::env;
use std::fs::{self};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process;
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use eyre::{bail, Context, Result};

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

/// Accepting communication from client and processing their messages.
fn listen_and_accept(address: String) {
    //FIXME error listening
    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Unable to listen: {e}");
            process::exit(1)
        }
    };

    let clients: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        println!("accepted new stream");

        {
            clients
                .lock()
                .unwrap()
                .insert(addr, stream.try_clone().unwrap());
        }

        let clients_clone = clients.clone();
        std::thread::spawn(move || loop {
            let mut reply: String;

            println!("trying to read incoming message");
            //let message = match incoming_message(&mut clients_lock[&addr].try_clone().unwrap()) {
            let message = match incoming_message(&mut stream) {
                Ok(msg) => msg,
                Err(e) => {
                    // complain if message is invalid
                    eprintln!("message has invalid format: {e}");
                    continue;
                }
            };
            println!("message read: {message:?}");
            let clients_lock = clients_clone.lock().unwrap();
            // broadcast to other clients here ↓
            drop(clients_lock);

            //FIXME notify user connect/disconnect

            match message {
                MessageType::Text(text) => {
                    println!("{} {addr}: {text:?}", current_time());
                    reply = "Received".into();
                }
                MessageType::File(name, content) => {
                    //TODO unable to save
                    //TODO file already exist
                    reply = format!("{} saving: {}/{}", current_time(), DIRECTORY_FILES, name);
                    println!("{reply}");
                    fs::write(format!("{}/{}", DIRECTORY_FILES, name), content)
                        .expect("Could not write file");
                }
                MessageType::Image(image) => {
                    //TODO unable to save
                    //TODO file already exist
                    let timestamp: String = std::time::UNIX_EPOCH
                        .elapsed()
                        .unwrap()
                        .as_secs()
                        .to_string();
                    reply = format!(
                        "{} saving: {}/{}.png",
                        current_time(),
                        DIRECTORY_IMAGES,
                        timestamp
                    );
                    println!("{reply}");
                    fs::write(format!("{}/{}.png", DIRECTORY_IMAGES, timestamp), &image)
                        .expect("Could not write file");
                }
            }
            let response = MessageType::Text(format!("{} 󰸞", reply));
            outgoing_message(&mut stream, &response);
            reply.clear();
        });
    }
}
