pub mod proto;
pub mod reader;
pub mod forwarder;
pub mod cmd;

use std::{net::TcpStream, io::BufWriter};
use std::io::Write;

use self::{proto::Proto, reader::BufioReader};

pub struct Conn {
    writer: BufWriter<TcpStream>,
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
            writer: BufWriter::new(stream),
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
        proto.encode(&mut self.writer).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn encode_bytes(&mut self, bytes: &[u8]) {
        match self.writer.write(bytes) {
            Err(e) => {
                println!("{:?}", e);
            }
            Ok(_) => {
                self.writer.flush().unwrap();
            },
        }
    }
}