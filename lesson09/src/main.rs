use lesson09::{deserialize_message, serialize_message, MessageType};
use std::collections::HashMap;
use std::env;
use std::fs::{self};
use std::io::{self, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

static FOLDER_FILES: &str = "files";
static FOLDER_IMAGES: &str = "images";

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

fn read_file(input: String) -> Vec<u8> {
    let mut filename = input.split(' ');
    let filename: &str = filename.nth(1).expect("missing filename");
    // println!(
    //                 "{} sending file {:?}",
    //                 std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
    //                 filename
    //             );
    std::fs::read(format!("./{}", filename)).unwrap()
}

fn server(address: &str) {
    println!(
        "{} Starting server!",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    );
    create_folder(FOLDER_FILES);
    create_folder(FOLDER_IMAGES);
    listen_and_accept(address)
}

fn client(address: &str) {
    println!(
        "{} Starting client!",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    );
    // TODO error message if client is not able to connect
    loop {
        println!(
            "{} What to send? (text / .image <filename> / .file <filename> / .quit): ",
            std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
        );

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();

        let message: MessageType = {
            if user_input.starts_with(".quit") {
                println!(
                    "{} Exiting!",
                    std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
                );
                break;
            } else if user_input.starts_with(".file") {
                let mut file = user_input.split(' ');
                let filename: &str = file.nth(1).expect("missing filename");
                MessageType::File(filename.to_string(), read_file(user_input.to_string()))
            } else if user_input.starts_with(".image") {
                MessageType::Image(read_file(user_input.to_string()))
            } else {
                MessageType::Text(user_input.to_string())
            }
        };

        let mut stream = TcpStream::connect(address).unwrap();
        send_message(&mut stream, &message);
        let response = incoming(stream);
        println!(
            "{} {response:?}",
            std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
        );
    }
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
        let message = incoming(clients.get(&addr).unwrap().try_clone().unwrap());

        match message {
            MessageType::Text(text) => {
                println!(
                    "{} {text:?}",
                    std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
                );
            }
            MessageType::File(name, content) => {
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
                //TODO convert to png
                fs::write(format!("{}/{}.png", FOLDER_IMAGES, timestamp), &image)
                    .expect("Could not write file");
            }
        }
        let response = MessageType::Text("Received".to_string());
        send_message(&mut stream, &response);
    }
}

fn incoming(mut stream: TcpStream) -> MessageType {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).unwrap();
    deserialize_message(&buffer)
}
