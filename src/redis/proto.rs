use std::{io::{Error, ErrorKind, }, vec, fmt, };
use futures::executor::block_on;

use log::{debug, log_enabled};
use tokio::io::{AsyncWriteExt, AsyncRead, BufWriter, AsyncWrite};

use super::reader::BufioReader;

type ProtoType = u8;
const UNKNOWN: ProtoType = 0;
const SIMPLE_STRING: ProtoType = b'+';
const ERROR: ProtoType = b'-';
const INTEGER: ProtoType = b':';
const BULK_STRING: ProtoType = b'$';
const ARRAY: ProtoType = b'*';

const CLRF: &[u8; 2] = b"\r\n";

// #[derive(Debug)]
pub struct Proto {
    pub proto_type: ProtoType,

    // bulk string: length
    // array: size
    pub len: i16,

    pub data: Vec<u8>,

    pub arr: Vec<Box<Proto>>,
}


impl Proto {
    pub fn new() -> Proto {
        Proto {
            proto_type: UNKNOWN,
            len: 0,
            data: vec![],
            arr: vec![],
        }
    }

    pub async fn decode(&mut self, reader: &mut BufioReader<impl AsyncRead+Unpin>) -> Result<&Proto, Error> {
        let mut line: Vec<u8> = vec![];
        let n = reader.read_clrf(&mut line).await?;
        if log_enabled!(log::Level::Debug) {
            debug!("{:?} read: {:?}", n, std::str::from_utf8(&line).unwrap());
        }
        self.proto_type = line[0];
        match self.proto_type {
            SIMPLE_STRING | ERROR | INTEGER => {
                self.data = line[1..line.len()-2].to_vec();
            }
            BULK_STRING => {
                self.decode_bulk_string(&line, reader).await?;
            }
            ARRAY => {
                self.decode_array(&line, reader).await?;
            }
            _ => {
                return Err(Error::new(ErrorKind::Other, "unknown proto type"));
            }
        }
        Ok(self)
    }

    pub async fn encode(&self, writer: &mut BufWriter<impl AsyncWrite+Unpin>) -> Result<(), Error> {
        match self.proto_type {
            SIMPLE_STRING | ERROR | INTEGER => {
                writer.write_all(&[self.proto_type]).await?;
                writer.write_all(&self.data).await?;
                writer.write_all(CLRF).await?;
                Ok(())
            }
            BULK_STRING => {
                writer.write_all(&[self.proto_type]).await?;
                writer.write_all(self.len.to_string().as_bytes()).await?;
                writer.write_all(CLRF).await?;
                if self.len > 0 {
                    writer.write_all(&self.data).await?;
                    writer.write_all(CLRF).await?;
                }
                Ok(())
            }
            ARRAY => {
                writer.write_all(&[self.proto_type]).await?;
                writer.write_all(self.len.to_string().as_bytes()).await?;
                writer.write_all(CLRF).await?;
                for p in &self.arr {
                    block_on(p.encode(writer))?;
                }
                Ok(())
            }
            _ => {
                Err(Error::new(ErrorKind::Other, "unknown proto type"))
            }
        }
    }

    async fn decode_bulk_string(&mut self, line: &Vec<u8>, reader: &mut BufioReader<impl AsyncRead+Unpin>) -> Result<&Proto, Error> {
        let len = num_from_bytes(&line[1..line.len()-2]);
        self.len = len;
        if len < 0 {
            return Ok(self);
        }
        let n = reader.read_n(len as u64, &mut self.data).await?;
        if (n as i16) < len {
            return Err(Error::new(ErrorKind::Other, "not enough data"));
        }
        reader.discard(2).await;
        Ok(self)
    }

    async fn decode_array(&mut self, line: &Vec<u8>, reader: &mut BufioReader<impl AsyncRead+Unpin>) ->  Result<&Proto, Error> {
        let array_size = num_from_bytes(&line[1..line.len()-2]);
        self.len = array_size;
        for _ in 0..array_size {
            let mut p_proto = Box::new(Proto::new());
            p_proto.as_mut().decode(reader).await?;
            self.arr.push(p_proto);
        }
        Ok(self)
    }

}

pub fn with_error(proto: &mut Proto, err: &str) {
    proto.proto_type = ERROR;
    proto.data = err.as_bytes().to_vec();
}

#[inline]
fn num_from_bytes(bytes: &[u8]) -> i16 {
    let mut data = bytes;
    let mut negative = false;
    if bytes[0] == b'-' {
        negative = true;
        data = &data[1..];
    }
    let mut num = 0;
    for b in data {
        let i = *b as i16;
        num = num * 10 + (i - b'0' as i16);
    }
    if negative {
        -num
    } else {
        num
    }
}

impl fmt::Debug for Proto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.proto_type {
            SIMPLE_STRING => {
                write!(f, "SIMPLE_STRING: {:?}", std::str::from_utf8(&self.data).unwrap())
            }
            ERROR => {
                write!(f, "ERROR: {:?}", std::str::from_utf8(&self.data).unwrap())
            }
            INTEGER => {
                write!(f, "INTEGER: {:?}", std::str::from_utf8(&self.data).unwrap())
            }
            BULK_STRING => {
                write!(f, "BULK_STRING: {:?}", std::str::from_utf8(&self.data).unwrap())
            }
            ARRAY => {
                write!(f, "ARRAY: {:?}", self.arr)
            }
            _ => {
                write!(f, "UNKNOWN: {:?}", self.data)
            }
        }
    }
}