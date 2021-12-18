use std::net::SocketAddr;
use crate::http::client::GameServerReq;

lazy_static::lazy_static!{
    pub static ref CFG:Cfg = Cfg::init();
}

#[derive(Clone,Debug)]
pub struct Cfg {
    pub addr:SocketAddr,
    pub static_dir:String,
    pub api_token:String,
    pub game_servers:Vec<GameServerReq>
}

impl Cfg{
    fn init()->Self{
        let mut cfg = Self::default();
        let mut settings = config::Config::default();
        settings
            // Add in `./Settings.toml`
            .merge(config::File::with_name("Settings"))
            .unwrap()
            // Add in settings from the environment (with a prefix of APP)
            .merge(config::Environment::with_prefix("TR"))
            .unwrap();
        
        if let Ok(port) = settings.get::<String>("port"){
            // TODO port错误处理
            let mut socket = String::from("0.0.0.0:");
            socket.push_str(&port);
            cfg.addr = socket.parse().unwrap();
        }
        if let Ok(static_dir) = settings.get::<String>("static_dir"){
            cfg.static_dir = static_dir;
        }
        if let Ok(api_token) = settings.get::<String>("api_token"){
            cfg.api_token = api_token;
        }
        if let Ok(game_servers)= settings.get::<Vec<GameServerReq>>("game_servers"){
            cfg.game_servers = game_servers;
        }
        cfg
    }
}

impl Default for Cfg{
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:8488".parse().unwrap(), 
            static_dir: "static".into(),
            api_token: "token".into(),
            game_servers:vec![]
        }
    }
}

