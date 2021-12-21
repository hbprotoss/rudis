pub mod proto;
pub mod reader;
pub mod forwarder;
pub mod cmd;

use std::net::TcpStream;
use std::io::Write;

use self::{proto::Proto, reader::BufioReader};

pub struct Conn<'a> {
    stream: &'a TcpStream,

    reader: BufioReader<&'a TcpStream>,
}

impl<'a> Conn<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self { 
            stream, 
            reader: BufioReader::new(stream),
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

    pub fn encode_bytes(&mut self, bytes: &[u8]) {
        match self.stream.write(bytes) {
            Err(e) => {
                println!("{:?}", e);
            }
            Ok(_) => {},
        }
    }
}