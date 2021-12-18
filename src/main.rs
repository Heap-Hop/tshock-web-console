use log::info;
use terraria_server_console::http::server::start_server;
use terraria_server_console::{logger_init, LevelFilter};
use terraria_server_console::cfg::CFG;

#[tokio::main]
async fn main() {
    info!("listen at {}",CFG.addr);
    // let cfg = Arc::new(Cfg::default());
    logger_init(LevelFilter::Debug);
    info!("Start！");
    start_server().await;
}