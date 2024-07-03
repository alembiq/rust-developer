#![warn(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/alembiq/rust-developer/issues/")]
//! Server for simple CLI chat

use std::collections::HashMap;
use std::env;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use eyre::{bail, Result};
use parking_lot::Mutex;
// use prometheus::{Gauge, Opts, Registry, Counter};
use lazy_static::lazy_static;
use prometheus::{self, register_int_counter, register_int_gauge, Encoder, IntCounter, IntGauge, TextEncoder};

use lesson18::{incoming_message, outgoing_message, server_address, timestamp, MessageType};

lazy_static! {
    static ref COUNTER_MESSAGE_TEXT: IntCounter =
        register_int_counter!("message_text", "Text messages delivered").unwrap();
    static ref COUNTER_MESSAGE_FILE: IntCounter =
        register_int_counter!("message_file", "Files delivered").unwrap();
    static ref COUNTER_MESSAGE_IMAGE: IntCounter =
        register_int_counter!("message_image", "Images delivered").unwrap();
    static ref COUNTER_CLIENTS: IntCounter =
        register_int_counter!("clients", "Clients over time").unwrap();
    static ref GAUGE_CLIENTS: IntGauge =
        register_int_gauge!("clients_current", "Active clients").unwrap();
}

fn main() -> Result<()> {
    let server_address: String = server_address(env::args().collect());
    println!("{} Starting server on {}!", timestamp(), server_address);

    // Register & measure some metrics.
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    // Gather the metrics.
    let metric_families = prometheus::gather();
    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let output = String::from_utf8(buffer.clone()).unwrap();
    println!("{output:?}");

    // Listen and process clients
    listen_and_accept(server_address)?;

    Ok(())
}

fn listen_and_accept(address: String) -> Result<()> {
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
        GAUGE_CLIENTS.inc();
        COUNTER_CLIENTS.inc();

        {
            clients.lock().insert(addr, stream.try_clone().unwrap());
        }

        let clients_clone = clients.clone();

        thread::spawn(move || loop {
            let message = match incoming_message(&mut stream) {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("{} {addr} stream interrupted: {e}", timestamp());
                    GAUGE_CLIENTS.dec();
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

                if let Err(e) = outgoing_message(peer_stream, &message) {
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
                    COUNTER_MESSAGE_TEXT.inc();
                }
                MessageType::File(name, _content) => {
                    println!("{} {addr} sending: {}", timestamp(), name);
                    COUNTER_MESSAGE_FILE.inc();
                }
                MessageType::Image(_image) => {
                    let filename: String = std::time::UNIX_EPOCH
                        .elapsed()
                        .unwrap()
                        .as_secs()
                        .to_string();
                    println!("{} {addr} sending: {}.png", timestamp(), filename);
                    COUNTER_MESSAGE_IMAGE.inc();
                }
            }
        });
    }
    Ok(())
}
