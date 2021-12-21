pub mod proto;
pub mod reader;
pub mod handler;
pub mod cmd;

use std::net::TcpStream;

use self::{proto::Proto, reader::BufioReader, handler::Handler, cmd::Command};

pub struct Conn<'a> {
    stream: &'a TcpStream,

    reader: BufioReader<&'a TcpStream>,
    handler: Handler<'a>,
}

impl<'a> Conn<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self { 
            stream, 
            reader: BufioReader::new(stream),
            handler: Handler::new(stream),
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
        self.handler.handle(cmd);
    }
}