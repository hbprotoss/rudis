
use std::{str::from_utf8, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use rand::Rng;

use crate::redis::proto::with_error;

use super::{cmd::Command, Conn};

pub struct Forwarder<H: Hasher> {
    node_conns: Vec<Conn>,
    hasher: H,
}

impl Forwarder<DefaultHasher> {
    pub fn new() -> Self { Self { 
        node_conns: vec![
            Conn::new("192.168.100.53".to_string(), 6379),
            Conn::new("192.168.100.53".to_string(), 6380),
        ],
        hasher: DefaultHasher::new(),
     } }

    pub fn forward(&mut self, cmd: &mut Command) {
        match cmd.name() {
            Some(name) => {
                println!("command: {:?}", from_utf8(name).unwrap());
                let conn: &mut Conn;
                match cmd.key() {
                    Some(key) => {
                        println!("key: {:?}", from_utf8(key).unwrap());
                        conn = self.determin_node(cmd.key().unwrap());
                    },
                    None => {
                        println!("key: none");
                        let mut rng = rand::thread_rng();
                        let index = rng.gen_range(0..self.node_conns.len());
                        conn = self.node_conns.get_mut(index).unwrap();
                    },
                }
                conn.encode(cmd.req);
                conn.decode(cmd.reply);
            }
            None => {
                with_error(cmd.reply, "ERR unknown command");
            }
        }
    }

    fn determin_node(&mut self, key: &Vec<u8>) -> &mut Conn {
        Hash::hash_slice(key, &mut self.hasher);
        let hash = self.hasher.finish();
        let size = self.node_conns.len();
        let index = hash as usize % size;
        println!("hash: {:?}, index: {:?}", hash, index);
        self.node_conns.get_mut(index).unwrap()
    }
}
