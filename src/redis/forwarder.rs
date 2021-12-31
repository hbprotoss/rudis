
use std::str::from_utf8;

use crate::redis::proto::with_error;

use super::cmd::Command;

pub struct Forwarder {
}

impl Forwarder {
    pub fn new() -> Self { Self {  } }

    pub fn forward(&mut self, cmd: &mut Command) {
        match cmd.name() {
            Some(name) => {
                println!("command: {:?}", from_utf8(name).unwrap());
                with_error(cmd.reply, "ERR unknown command");
            }
            None => {}
        }
    }
}
