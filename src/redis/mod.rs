pub mod proto;
pub mod reader;
pub mod forwarder;
pub mod cmd;

use std::net::TcpStream;
use std::io::Write;

use self::{proto::Proto, reader::BufioReader};

pub struct Conn {
    stream: TcpStream,

    reader: BufioReader<TcpStream>,
}

impl Conn {
    pub fn new(host: String, port: u16) -> Self {
        let stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
        Conn::new_from_tcp_stream(stream)
    }

    pub fn new_from_tcp_stream(stream: TcpStream) -> Self {
        let read_stream = stream.try_clone().expect("Failed to clone stream");
        Self { 
            stream, 
            reader: BufioReader::new(read_stream),
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

    pub fn encode(&mut self, proto: &Proto) {
        proto.encode(&mut self.stream).unwrap();
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