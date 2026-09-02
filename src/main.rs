use log::{debug, info};

mod config;
mod helper;
use config::Cfg;

mod backend;
mod clinet;

mod router;

const CONFIG_DIR: &str = "config/config.toml";

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Start loadway");
    let cfg = Cfg::load(CONFIG_DIR).expect("Error from load config");

    debug!("\n{:#?}", cfg)
}
