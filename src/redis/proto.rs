use std::{io::{Error, ErrorKind, Read}, };

use super::reader::BufioReader;

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
    pub fn decode(&mut self, reader: &mut BufioReader<impl Read>) -> Result<&Proto, Error> {
        let line = reader.read_clrf()?;
        self.proto_type = line[0];
        match self.proto_type {
            SIMPLE_STRING | ERROR | INTEGER => {
                self.data = line[1..line.len()-2].to_vec();
            }
            BULK_STRING => {
                self.decode_bulk_string(line, reader)?;
            }
            ARRAY => {
                let len = line[1..].parse::<usize>().unwrap();
                self.arr = Vec::with_capacity(len);
                for _ in 0..len {
                    self.arr.push(self.decode(reader)?);
                }
            }
            _ => {
                return Err(Error::new(ErrorKind::Other, "unknown proto type"));
            }
        }
        Ok(self)
    }

    fn decode_bulk_string(&self, line: &Vec<u8>, reader: &mut BufioReader<impl Read>) -> Result<&Proto, Error> {
        let len = num_from_bytes(&line[1..line.len()-2]);
    }
}

fn num_from_bytes(bytes: &[u8]) -> u16 {
    let mut num = 0;
    for b in bytes {
        num = num * 10 + (*b - b'0') as u16;
    }
    num
}
