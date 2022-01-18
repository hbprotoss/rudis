mod redis;
mod rudis_config;

use config::{Config, File};
use redis::server::Server;
use rudis_config::RudisConfig;

#[tokio::main]
pub async fn main() {
    let rc = init_config();
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let mut s = Server::new(rc);
    s.serve().await;
}

fn init_config() -> RudisConfig {
    let mut settings = Config::default();
    let _ = settings
        .merge(File::with_name("config/rudis.yml"))
        ;
    settings.try_into::<RudisConfig>().unwrap()
}