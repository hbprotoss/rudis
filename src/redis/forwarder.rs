
use std::str::from_utf8;

use super::Conn;
use super::cmd::Command;

pub struct Forwarder<'a> {
    proxy_conn: &'a mut Conn<'a>,
}

impl<'a> Forwarder<'a> {
    pub fn new(proxy_conn: &'a mut Conn<'a>) -> Self {
        let ret = Self { proxy_conn };
        ret
    }

    pub fn forward(&mut self, cmd: &Command) {
        match cmd.name() {
            Some(name) => {
                println!("command: {:?}", from_utf8(name).unwrap());
                self.proxy_conn.encode_bytes(b"-ERR unknown command\r\n");
            }
            None => {}
        }
    }
}
