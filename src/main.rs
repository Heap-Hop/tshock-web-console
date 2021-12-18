use log::info;
use tshock_web_console::http::server::start_server;
use tshock_web_console::{logger_init, LevelFilter};
use tshock_web_console::cfg::CFG;

#[tokio::main]
async fn main() {
    info!("listen at {}",CFG.addr);
    logger_init(LevelFilter::Debug);
    info!("Start！");
    start_server().await;
}