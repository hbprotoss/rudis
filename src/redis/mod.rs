mod proto;

use std::{net::TcpStream, io::{BufReader, BufRead}};

use self::proto::Proto;

pub struct Conn<'a> {
    stream: &'a TcpStream,

    r: BufReader<&'a TcpStream>,
}

impl<'a> Conn<'a> {
    pub fn new(stream: &'a TcpStream) -> Self { 
        Self { 
            stream, 
            r: BufReader::new(stream),
        }
    }

    pub fn decode(&self, proto: &mut Proto<'a>) {
    }
}