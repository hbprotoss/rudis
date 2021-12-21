pub mod proto;
pub mod reader;
pub mod handler;
pub mod cmd;

use std::net::TcpStream;

use self::{proto::Proto, reader::BufioReader, handler::Forwarder, cmd::Command};

pub struct Conn<'a> {
    stream: &'a TcpStream,

    reader: BufioReader<&'a TcpStream>,
    forward: Forwarder<'a>,
}

impl<'a> Conn<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self { 
            stream, 
            reader: BufioReader::new(stream),
            forward: Forwarder::new(stream),
        }
    }

    pub fn decode(&mut self, proto: &mut Proto) {
        match proto.decode(&mut self.reader) {
            Err(e) => {
                println!("{:?}", e);
            }
            Ok(_) => {},
        }
    }

    pub fn handle(&mut self, cmd: &Command) {
        self.forward.forward(cmd);
    }
}