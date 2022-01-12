use tokio::net::{TcpListener, TcpStream};
use std::io::Result;


use log::{debug, info};

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
    pub async fn serve(&mut self) {
        let listener = TcpListener::bind("0.0.0.0:3333").await.unwrap();
        // accept connections and process them, spawning a new thread for each one
        info!("Server listening on port 3333");
        tokio::select! {
            _ = self.run(listener) => {
                info!("Server stopped");
            }
        }
        
    }

    async fn run(& self, listener: TcpListener) -> Result<()> {
        loop {
            let (stream, _) = listener.accept().await?;
            info!("New connection: {}", stream.peer_addr()?);
            tokio::spawn(async move {
                handle_client(stream).await;
            });
        }
    }
}

async fn handle_client(stream: TcpStream) {
    let mut conn = Conn::new_from_tcp_stream(stream).await;
    let mut forwarder = Forwarder::new().await;
    loop {
        let mut req = Proto::new();
        conn.decode(&mut req).await;
        debug!("req: {:?}", req);
        let mut reply = Proto::new();
        let mut command = Command::new(&mut req, &mut reply);
        forwarder.forward(&mut command).await;
        conn.encode(&reply).await;
    }
}