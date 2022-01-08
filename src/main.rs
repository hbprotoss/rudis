use redis::Server;

mod redis;



fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let mut s = Server::new();
    s.serve();
}
