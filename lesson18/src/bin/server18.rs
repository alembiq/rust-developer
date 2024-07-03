#![warn(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/alembiq/rust-developer/issues/")]
//! Server for simple CLI chat

use std::collections::HashMap;
use std::env;
use std::io::Result as IoResult;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use eyre::{bail, Result};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use prometheus::{
    self, gather, register_int_counter, register_int_gauge, Encoder, IntCounter, IntGauge,
    Registry, TextEncoder,
};

use lesson18::{incoming_message, outgoing_message, server_address, timestamp, MessageType};

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
    static ref COUNTER_MESSAGE_TEXT: IntCounter =
        register_int_counter!("message_text", "Text messages delivered").unwrap();
    static ref COUNTER_MESSAGE_FILE: IntCounter =
        register_int_counter!("message_file", "Files delivered").unwrap();
    static ref COUNTER_MESSAGE_IMAGE: IntCounter =
        register_int_counter!("message_image", "Images delivered").unwrap();
    static ref COUNTER_CLIENTS: IntCounter =
        register_int_counter!("clients_runtime", "Clients over time").unwrap();
    static ref GAUGE_CLIENTS: IntGauge =
        register_int_gauge!("clients_current", "Active clients").unwrap();
}

/// Generates Prometheus metrics
async fn metrics() -> impl Responder {
    let encoder = TextEncoder::new();
    let mut buf = vec![];

    if let Err(e) = encoder.encode(&gather(), &mut buf) {
        eprintln!("Failed to collect metrics: {e}");
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Failed to collect metrics!");
    }
    HttpResponse::build(StatusCode::OK).body(String::from_utf8(buf).unwrap())
}

/// Webserver for /metrics
async fn metrics_http() -> IoResult<()> {
    let _ = HttpServer::new(|| App::new().route("/metrics", web::get().to(metrics)))
        .bind("0.0.0.0:11112")
        .expect("Failed to bind metrics endpoint")
        .run()
        .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let server_address: String = server_address(env::args().collect());
    println!("{} Starting server on {}!", timestamp(), server_address);

    let metrics_handle = tokio::spawn(async {
        let _ = metrics_http().await;
    });
    let server_handle = tokio::spawn(async {
        let _ = listen_and_accept(server_address);
    });

    metrics_handle.await.unwrap();
    server_handle.await.unwrap();

    //TODO find a way how to terminate the server ;)
    // listen_and_accept(server_address);

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
                    // TODO announce disconnected client to everyone and remove everywhere?
                    GAUGE_CLIENTS.dec();
                    break;
                }
            };

            // message other clients
            let mut clients_lock = clients_clone.lock();
            let peers_to_remove = vec![];
            for (peer_addr, peer_stream) in clients_lock.iter_mut() {
                if *peer_addr == addr {
                    continue;
                }

                if let Err(e) = outgoing_message(peer_stream, &message) {
                    eprintln!("{} failed to send message: {message:?} -> {e}", timestamp());
                }
            }
            // remove dead clients
            for peer_addr in peers_to_remove {
                clients_lock.remove(&peer_addr);
                println!("remove dead client {}", &peer_addr);
            }

            drop(clients_lock);

            //MESSAGE SNEAKPEAK and COUNTERS
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
