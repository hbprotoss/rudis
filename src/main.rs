mod redis;

use redis::server::Server;

#[tokio::main]
pub async fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let mut s = Server::new();
    s.serve().await;
}
