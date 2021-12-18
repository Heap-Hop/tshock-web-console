use hyper::{Client, body::Buf};
use log::info;


// pub async fn get_local() {
//     // Still inside `async fn main`...
//     let client = Client::new();

//     // Parse an `http::Uri`...
//     let uri = "http://127.0.0.1:8488".parse().unwrap();
//     // Await the response...
//     if let Ok(resp) = client.get(uri).await {
//         info!("Response: {}", resp.status());
//     } else {
//         info!("nothing");
//     }
// }

pub async fn get_terraria_info(game_servers: &Vec<GameServerReq>)->Vec<GameServerRes> {
    // let host = "http://sh.breakbeat.cn";
    let host = "http://localhost";
    let client = Client::new();
    let mut res:Vec<GameServerRes> = Vec::new();
    for server in game_servers {
        let uri = format!(
            "{}:{}/v2/server/status?token={}",
            host, server.port, server.token
        )
        .parse()
        .unwrap();
        if let Ok(resp) = client.get(uri).await {
            let body = hyper::body::aggregate(resp).await.unwrap();
            // let body = hyper::body::to_bytes(resp).await.unwrap();
            let server_status:ServerStatus = serde_json::from_reader(body.reader()).unwrap(); // TODO error handle
            info!("Response: {:?}", server_status);
            res.push(GameServerRes{port:server.port,status:server_status})
        } else {
            info!("nothing");
        }
    }
    res
}

use serde::{Deserialize, Serialize};

#[derive(Clone,Debug,Deserialize)]
pub struct GameServerReq{
    pub port:u16,
    pub token:String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GameServerRes{
    port:u16,
    status:ServerStatus
}

#[derive(Serialize,Deserialize, Debug)]
pub struct ServerStatus {
    status: String,
    name: String,
    serverversion:String,
    // #[allow(unused)]
    tshockversion:TshockVersion,
    port:u32,
    playercount:u8,
    maxplayers:u8,
    world:String,
    uptime:String,
    serverpassword:bool
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize, Debug)]
struct TshockVersion{
    Major: u16,
    Minor: u16,
    Build: u16,
    Revision: u16,
    MajorRevision: u16,
    MinorRevision: u16
}

// {
//     "status": "200",
//     "name": "",
//     "serverversion": "v1.4.3.2",
//     "tshockversion": {
//       "Major": 4,
//       "Minor": 5,
//       "Build": 10,
//       "Revision": 0,
//       "MajorRevision": 0,
//       "MinorRevision": 0
//     },
//     "port": 7777,
//     "playercount": 0,
//     "maxplayers": 8,
//     "world": "Solaire",
//     "uptime": "12.00:06:41",
//     "serverpassword": true
//   }
