use std::{io::BufReader, net::TcpStream};

struct Buffer<'a> {
    buf: &'a mut [u8],
    r: usize,
    w: usize,
}

type ProtoType = u8;
const SIMPLE_STRING: ProtoType = b'+';
const ERROR: ProtoType = b'-';
const INTEGER: ProtoType = b':';
const BULK_STRING: ProtoType = b'$';
const ARRAY: ProtoType = b'*';

pub struct Proto<'a> {
    proto_type: ProtoType,

    // bulk string: length
    // array: size
    count: u16,

    arr: [&'a Proto<'a>],
}

pub struct Command<'a> {
    req: &'a Proto<'a>,
    reply: &'a Proto<'a>,
}

const BUF_SIZE: usize = 32;
const GROW_FACTOR: usize = 2;
pub struct BufioReader<'a> {
    reader: BufReader<&'a TcpStream>,

    buf: Vec<u8>,
    r: usize,
    w: usize,
}

impl<'a> BufioReader<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self {
            reader: BufReader::new(stream),
            buf: vec![0; BUF_SIZE],
            r: 0,
            w: 0,
        }
    }

    pub fn read_clrf(&self) {
        self.reader.read_vectored(self.buf);
    }
}
