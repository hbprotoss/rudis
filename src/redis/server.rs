use std::net::{TcpListener, TcpStream};
use std::thread;

use log::{debug, info, error};

use crate::redis::cmd::Command;
use crate::redis::proto::Proto;

use super::conn::Conn;
use super::forwarder::Forwarder;


pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }
}

impl Server {
    pub fn serve(&mut self) {
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        // accept connections and process them, spawning a new thread for each one
        info!("Server listening on port 3333");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(|| {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
                Err(e) => {
                    error!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
        // close the socket server
        drop(listener);
    }
}

fn handle_client(stream: TcpStream) {
    let mut conn = Conn::new_from_tcp_stream(stream);
    let mut forwarder = Forwarder::new();
    loop {
        let mut req = Proto::new();
        conn.decode(&mut req);
        debug!("req: {:?}", req);
        let mut reply = Proto::new();
        let mut command = Command::new(&mut req, &mut reply);
        forwarder.forward(&mut command);
        conn.encode(&reply);
    }
}