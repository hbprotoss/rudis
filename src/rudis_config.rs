use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct RudisConfig {
    pub listen: String,
    pub port: u16,
    pub proxy_type: String,
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub host: String,
    pub port: u16,
}