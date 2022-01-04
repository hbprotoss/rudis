
use std::str::from_utf8;

use crate::redis::proto::with_error;

use super::{cmd::Command, Conn};

pub struct Forwarder {
    node_conn: Conn,
}

impl Forwarder {
    pub fn new() -> Self { Self { 
        node_conn: Conn::new("192.168.100.53".to_string(), 6379),
     } }

    pub fn forward(&mut self, cmd: &mut Command) {
        match cmd.name() {
            Some(name) => {
                println!("command: {:?}", from_utf8(name).unwrap());
                // with_error(cmd.reply, "ERR unknown command");
                self.node_conn.encode(cmd.req);
                self.node_conn.decode(cmd.reply);
            }
            None => {}
        }
    }
}
