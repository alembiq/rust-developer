use std::collections::HashMap;
use std::env;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use eyre::{bail, Result};
use parking_lot::Mutex;

use shared13::{current_time, incoming_message, outgoing_message, server_address, MessageType};

fn main() -> Result<()> {
    let server_address: String = server_address(env::args().collect());
    println!("{} Starting server on {}!", current_time(), server_address);
    listen_and_accept(server_address)?;
    Ok(())
}

fn listen_and_accept(address: String) -> Result<()> {
    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => {
            bail!("{} Unable to listen: {e}", current_time())
        }
    };

    let clients: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        println!("{} {} stream started", current_time(), addr);

        {
            clients.lock().insert(addr, stream.try_clone().unwrap());
        }

        let clients_clone = clients.clone();

        thread::spawn(move || loop {
            let mut clients_lock = clients_clone.lock();
            let mut peers_to_remove = vec![];
            let message = match incoming_message(&mut stream) {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("{} {addr} stream interrupted: {e}", current_time());
                    break;
                }
            };

            // message other clients
            for (peer_addr, peer_stream) in clients_lock.iter_mut() {
                if *peer_addr == addr {
                    continue;
                }

                if let Err(e) = outgoing_message(peer_stream, &message) {
                    eprintln!(
                        "{} failed to send message: {message:?} -> {e}",
                        current_time()
                    );
                    peers_to_remove.push(peer_addr.clone());
                }
            }

            for peer_addr in peers_to_remove {
                clients_lock.remove(&peer_addr);
            }

            drop(clients_lock);

            //MESSAGE SNEAKPEAK
            match message {
                MessageType::Text(text) => {
                    println!("{} {addr}: {text:?}", current_time());
                }
                MessageType::File(name, _content) => {
                    println!("{} {addr} sending: {}", current_time(), name);
                }
                MessageType::Image(_image) => {
                    let timestamp: String = std::time::UNIX_EPOCH
                        .elapsed()
                        .unwrap()
                        .as_secs()
                        .to_string();
                    println!("{} {addr} sending: {}.png", current_time(), timestamp);
                }
            }
        });
    }
    Ok(())
}
