use std::{
    io::{BufRead, BufReader, Error, Read},
    ops::Deref,
};

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
        let total: usize = 0;
        loop {
            match self.reader.read_until(b'\n', buf) {
                Ok(n) => {
                    total += n;
                    if n == 0 || buf[n - 1] == b'\r' {
                        return Ok(total);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

impl<R> Deref for BufioReader<R> {
    type Target = BufReader<R>;

    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}
