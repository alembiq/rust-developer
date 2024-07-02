use eyre::{bail, Result};
use parking_lot::Mutex;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
// use tokio::sync::broadcast;

use lesson15::{message_incoming, message_outgoing, server_address, timestamp, MessageType};

use std::collections::HashMap;
use std::env;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

#[tokio::main]
async fn main() -> Result<()> {
    let server_address: String = server_address(env::args().collect());
    println!("{} Starting server on {}!", timestamp(), server_address);
    listen_and_accept(server_address).await?;
    Ok(())
}

async fn listen_and_accept(address: String) -> Result<()> {
    //     let listener = TcpListener::bind(&address)
    //         .await
    //         .context("Unable to listen")?;
    //     println!("{} Server running on {}", timestamp(), address);

    //     loop {
    //         // Accept incoming connections
    //         let (mut socket, addr) = match listener.accept().await {
    //             Ok((socket, addr)) => (socket, addr),
    //             Err(e) => {
    //                 eprintln!("{} Failed to accept connection: {}", timestamp(), e);
    //                 continue;
    //             }
    //         };
    //         println!("{} New connection from {}", timestamp(), addr);

    //         // Spawn a new task for each connection
    //         tokio::spawn(async move {
    //             let mut buffer = [0; 1024];
    //             // Read data from the socket
    //             loop {
    //                 match socket.read(&mut buffer).await {
    //                     Ok(0) => {
    //                         // Connection was closed
    //                         println!("{} Connection closed {}", timestamp(), addr);
    //                         return;
    //                     }
    //                     Ok(n) => {
    //                         //TODO save received data to db
    //                         //TODO forward to all connected clients
    //                         println!("{} {} data", timestamp(), addr);
    //                         // Echo the data back to the client
    //                         if let Err(e) = socket.write_all(&buffer[..n]).await {
    //                             eprintln!("{} Failed to write to socket: {}", timestamp(), e);
    //                             return;
    //                         }
    //                     }
    //                     Err(e) => {
    //                         eprintln!("{} Failed to read from socket: {}", timestamp(), e);
    //                         return;
    //                     }
    //                 }
    //             }
    //         });
    //     }
    // }
    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => {
            bail!("{} Unable to listen: {e}", timestamp())
        }
    };

    let clients: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        println!("{} {} stream started", timestamp(), addr);

        {
            clients.lock().insert(addr, stream.try_clone().unwrap());
        }

        let clients_clone = clients.clone();

        thread::spawn(move || loop {
            let message = match message_incoming(&mut stream) {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("{} {addr} stream interrupted: {e}", timestamp());
                    break;
                }
            };

            // message other clients
            let mut clients_lock = clients_clone.lock();
            let mut peers_to_remove = vec![];
            for (peer_addr, peer_stream) in clients_lock.iter_mut() {
                if *peer_addr == addr {
                    continue;
                }

                if let Err(e) = message_outgoing(peer_stream, &message) {
                    eprintln!("{} failed to send message: {message:?} -> {e}", timestamp());
                    peers_to_remove.push(*peer_addr);
                }
            }

            for peer_addr in peers_to_remove {
                clients_lock.remove(&peer_addr);
            }

            drop(clients_lock);

            //MESSAGE SNEAKPEAK
            match message {
                MessageType::Text(text) => {
                    println!("{} {addr}: {text:?}", timestamp());
                }
                MessageType::File(name, _content) => {
                    println!("{} {addr} sending: {}", timestamp(), name);
                }
                MessageType::Image(_image) => {
                    let file_name: String = std::time::UNIX_EPOCH
                        .elapsed()
                        .unwrap()
                        .as_secs()
                        .to_string();
                    println!("{} {addr} sending: {}.png", timestamp(), file_name);
                }
            }
        });
    }
    Ok(())
}
