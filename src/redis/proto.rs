use std::{io::{Error, ErrorKind, Read}, vec, };

use super::reader::BufioReader;

struct Buffer<'a> {
    buf: &'a mut [u8],
    r: usize,
    w: usize,
}

type ProtoType = u8;
const UNKNOWN: ProtoType = 0;
const SIMPLE_STRING: ProtoType = b'+';
const ERROR: ProtoType = b'-';
const INTEGER: ProtoType = b':';
const BULK_STRING: ProtoType = b'$';
const ARRAY: ProtoType = b'*';

#[derive(Debug)]
pub struct Proto {
    proto_type: ProtoType,

    // bulk string: length
    // array: size
    data: Vec<u8>,

    arr: Vec<Box<Proto>>,
}

pub struct Command<'a> {
    req: &'a Proto,
    reply: &'a Proto,
}

impl Proto {
    pub fn new() -> Proto {
        Proto {
            proto_type: UNKNOWN,
            data: vec![],
            arr: vec![],
        }
    }

    pub fn decode(&mut self, reader: &mut BufioReader<impl Read>) -> Result<&Proto, Error> {
        let mut line: Vec<u8> = vec![];
        let n = reader.read_clrf(&mut line)?;
        println!("{:?} read: {:?}", n, std::str::from_utf8(&line).unwrap());
        self.proto_type = line[0];
        match self.proto_type {
            SIMPLE_STRING | ERROR | INTEGER => {
                self.data = line[1..line.len()-2].to_vec();
            }
            BULK_STRING => {
                self.decode_bulk_string(&line, reader)?;
            }
            ARRAY => {
                self.decode_array(&line, reader)?;
            }
            _ => {
                return Err(Error::new(ErrorKind::Other, "unknown proto type"));
            }
        }
        Ok(self)
    }

    fn decode_bulk_string(&mut self, line: &Vec<u8>, reader: &mut BufioReader<impl Read>) -> Result<&Proto, Error> {
        let len = num_from_bytes(&line[1..line.len()-2]);
        let n = reader.read_n(len as u64, &mut self.data)?;
        if (n as u16) < len {
            return Err(Error::new(ErrorKind::Other, "not enough data"));
        }
        reader.discard(2);
        Ok(self)
    }

    fn decode_array(&mut self, line: &Vec<u8>, reader: &mut BufioReader<impl Read>) -> Result<&Proto, Error> {
        let array_size = num_from_bytes(&line[1..line.len()-2]);
        for _ in 0..array_size {
            let mut p_proto = Box::new(Proto::new());
            p_proto.as_mut().decode(reader)?;
            self.arr.push(p_proto);
        }
        Ok(self)
    }
}

fn num_from_bytes(bytes: &[u8]) -> u16 {
    let mut num = 0;
    for b in bytes {
        num = num * 10 + (*b - b'0') as u16;
    }
    num
}
