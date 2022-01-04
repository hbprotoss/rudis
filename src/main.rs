mod redis;

use std::net::{TcpListener, TcpStream};
use std::thread;

use redis::forwarder::Forwarder;
use redis::proto::Proto;
use redis::Conn;
use redis::cmd::Command;

fn handle_client(stream: TcpStream) {
    let mut conn = Conn::new_from_tcp_stream(stream);
    let mut forwarder = Forwarder::new();
    loop {
        let mut req = Proto::new();
        conn.decode(&mut req);
        println!("{:?}", req);
        let mut reply = Proto::new();
        let mut command = Command::new(&mut req, &mut reply);
        forwarder.forward(&mut command);
        conn.encode(&reply);
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
