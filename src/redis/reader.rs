use std::{
    io::{BufRead, BufReader, Error, Read, ErrorKind},
};

const SEEK_BUF_SIZE: usize = 8;

pub struct BufioReader<R> {
    reader: BufReader<R>,
}

impl<R: Read> BufioReader<R> {
    pub fn new(r: R) -> Self {
        Self {
            reader: BufReader::new(r),
        }
    }

    pub fn read_clrf(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let mut total: usize = 0;
        loop {
            match self.reader.read_until(b'\n', buf) {
                Ok(n) => {
                    if n == 0 {
                        return Err(Error::new(ErrorKind::Other, "EOF"));
                    }
                    total += n;
                    if buf[n - 2] == b'\r' {
                        return Ok(total);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub fn discard(&mut self, offset: u64) -> u64 {
        let mut t = self.reader.by_ref().take(offset);
        let mut buf = [0 as u8; SEEK_BUF_SIZE];
        let mut n = 0 as usize;
        loop {
            let nread = t.read(&mut buf).unwrap();
            n += nread;
            if nread != SEEK_BUF_SIZE {
                break;
            }
        }
        n as u64
    }

    pub fn read_n(&mut self, n: u64, buf: &mut Vec<u8>) -> Result<usize, Error> {
        self.reader.by_ref().take(n).read_to_end(buf)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.reader.read(buf)
    }
}

// impl<R> Deref for BufioReader<R> {
//     type Target = BufReader<R>;

//     fn deref(&self) -> &Self::Target {
//         &self.reader
//     }
// }
