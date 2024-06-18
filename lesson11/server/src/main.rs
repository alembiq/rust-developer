use shared::{incoming_message, is_valid_ip, outgoing_message, MessageType};
use std::collections::HashMap;
use std::env;
use std::fs::{self};
use std::net::{SocketAddr, TcpListener, TcpStream};

static FOLDER_FILES: &str = "files";
static FOLDER_IMAGES: &str = "images";

fn main() {
    let default_address = "127.0.0.1:11111";

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "help" {
        println!("=============== USAGE ===============");
        println!("{} listenerIP:port", args[0]);
    } else {
        server(if args.len() > 1 && is_valid_ip(&args[1]) {
            &args[1]
        } else {
            default_address
        });
    }
}

fn server(address: &str) {
    println!(
        "{} Starting server!",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    );
    //create folders to store incomming objects
    create_folder(FOLDER_FILES);
    create_folder(FOLDER_IMAGES);
    listen_and_accept(address)
}

// Accepting communication from client and processing their messages.
fn listen_and_accept(address: &str) {
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
                println!(
                    "{} {text:?}",
                    std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
                );
            }
            MessageType::File(name, content) => {
                //FILE transfer
                println!(
                    "{} saving: {}/{}",
                    std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
                    FOLDER_FILES,
                    name
                );
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
                    std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
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

fn create_folder(folder: &str) {
    if let Err(why) = fs::create_dir(folder) {
        println!(
            "{} creating {} folder: {:?}",
            std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
            { folder },
            why.kind()
        )
    }
}
