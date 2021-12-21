mod proto;
mod reader;

use std::{net::TcpStream};

use self::{proto::{Proto}, reader::BufioReader};

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
    }
}