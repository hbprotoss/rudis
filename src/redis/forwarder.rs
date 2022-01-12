
use std::{str::from_utf8, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use log::{debug, error, log_enabled};
use rand::Rng;

use crate::redis::proto::with_error;

use super::{cmd::Command, conn::{Conn, }, };

pub struct Forwarder {
    node_conns: Vec<Conn>,
}

impl Forwarder {
    pub async fn new() -> Self { Self { 
        node_conns: vec![
            Conn::new("192.168.100.53".to_string(), 6379).await,
            Conn::new("192.168.100.53".to_string(), 6380).await,
        ],
     } }

    pub async fn forward<'a>(&mut self, cmd: &mut Command<'a>) {
        match cmd.name() {
            Some(name) => {
                if log_enabled!(log::Level::Debug) {
                    debug!("forward: {:?}", from_utf8(name).unwrap());
                }
                let conn: &mut Conn;
                match cmd.key() {
                    Some(key) => {
                        if log_enabled!(log::Level::Debug) {
                            debug!("key: {:?}", from_utf8(key).unwrap());
                        }
                        conn = self.determin_node(cmd.key().unwrap());
                    },
                    None => {
                        error!("key: none");
                        let mut rng = rand::thread_rng();
                        let index = rng.gen_range(0..self.node_conns.len());
                        conn = self.node_conns.get_mut(index).unwrap();
                    },
                }
                conn.encode(cmd.req).await;
                conn.decode(cmd.reply).await;
            }
            None => {
                with_error(cmd.reply, "ERR unknown command");
            }
        }
    }

    fn determin_node(&mut self, key: &Vec<u8>) -> &mut Conn {
        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(key, &mut hasher);
        let hash = hasher.finish();
        let size = self.node_conns.len();
        let index = hash as usize % size;
        debug!("hash: {:?}, index: {:?}", hash, index);
        self.node_conns.get_mut(index).unwrap()
    }
}
