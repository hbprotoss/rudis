use std::io::Write;
use std::{net::TcpStream, str::from_utf8};

use super::cmd::Command;

pub struct Handler<'a> {
    stream: &'a TcpStream,
}

impl<'a> Handler<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self { stream }
    }

    pub fn handle(&mut self, cmd: &Command) {
        match cmd.name() {
            Some(name) => {
                println!("command: {:?}", from_utf8(name).unwrap());
                match self.stream.write(b"-ERR unknown command\r\n") {
                    Ok(n) => {
                        println!("wrote {} bytes", n);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            None => {}
        }
    }
}
