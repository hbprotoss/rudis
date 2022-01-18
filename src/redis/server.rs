use std::io::Result;
use tokio::net::{TcpListener, TcpStream};

use log::{debug, info};

use crate::redis::cmd::Command;
use crate::redis::proto::Proto;
use crate::rudis_config::RudisConfig;

use super::conn::Conn;
use super::forwarder::Forwarder;

pub struct Server {
    pub rc: RudisConfig,
}

impl Server {
    pub fn new(rc: RudisConfig) -> Self {
        Self { rc }
    }
}

impl Server {
    pub async fn serve(&mut self) {
        let addr = format!("{}:{}", self.rc.listen, self.rc.port);
        let listener = TcpListener::bind(&addr).await.unwrap();
        // accept connections and process them, spawning a new thread for each one
        info!("Server listening on {}", &addr);
        tokio::select! {
            _ = self.run(listener) => {
                info!("Server stopped");
            }
        }
    }

    async fn run(&self, listener: TcpListener) -> Result<()> {
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
