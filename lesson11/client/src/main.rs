use shared::{incoming_message, is_valid_ip, outgoing_message, MessageType};
use std::env;
use std::io::{self, Cursor};
use std::net::TcpStream;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;

fn main() {
    let default_address = "127.0.0.1:11111";

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "help" {
        println!("=============== USAGE ===============");
        println!("{} serverIP:port", args[0]);
    } else {
        client(if args.len() > 1 && is_valid_ip(&args[1]) {
            &args[1]
        } else {
            default_address
        });
    }
}

fn read_file(input: String) -> Vec<u8> {
    let mut filename = input.split(' ');
    let filename: &str = filename.nth(1).expect("missing filename");
    std::fs::read(format!("./{}", filename)).unwrap()
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
                let mut file = user_input.split(' ');
                let filename: &str = file.nth(1).expect("missing filename");
                let img = image::open(filename).unwrap();
                let mut output = Cursor::new(Vec::new());
                let encoder = PngEncoder::new(&mut output);
                let _ = encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color().into());
                MessageType::Image(output.into_inner() as  Vec<u8>)
            } else {
                MessageType::Text(user_input.to_string())
            }
        };

        let mut stream = TcpStream::connect(address).unwrap();
        outgoing_message(&mut stream, &message);
        let response = incoming_message(stream);
        println!(
            "{} {response:?}",
            std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
        );
    }
}
