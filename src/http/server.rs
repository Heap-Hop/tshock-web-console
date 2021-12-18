use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Result, Server, StatusCode,
};
use log::{debug, info};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::cfg::CFG;

use super::client::get_terraria_info;

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not found".into())
        .unwrap()
}

// server
pub async fn start_server() {
    let make_service = make_service_fn(move |_| {
        // 这里如果传入一个拥有所有权的参数，则需要在这里复制，闭包和async闭包之间
        async move {
            let make_service = service_fn(move |req| request_handle(req));
            Ok::<_, hyper::Error>(make_service)
        }
    });
    let server = Server::bind(&CFG.addr).serve(make_service);
    info!("Listening on http://{}", &CFG.addr);
    server.await.unwrap();
}

// 注意 测试异步性的时候不要用chrome，chrome同一个地址用的是同一个tcp，会造成阻塞的现象，用chrome和firefox同时测试即可测试出真正的异步性
async fn request_handle(req: Request<Body>) -> Result<Response<Body>> {
    let req_url = url::Url::parse(&format!("http://127.0.0.1{}", req.uri())).unwrap(); // TODO 错误处理 以及更合理的用法
    let segments = req_url
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap(); // TODO 错误处理

    match (req.method(), segments[0]) {
        (&Method::GET, "api") => match api_get(req.uri().query().unwrap_or("") ,segments).await {
            ApiRes::Success(res) => Ok(Response::new(res.into())),
            ApiRes::TokenFail => Ok(Response::new("token fail".into())),
            ApiRes::Null => Ok(not_found()),
        },
        (&Method::GET, _) => {
            let path = req.uri().path();
            debug!("read file:{}", path);
            // TODO 改用Cow
            if path == "/" {
                file_sender(format!("{}{}", CFG.static_dir, "/index.html")).await
            } else {
                file_sender(format!("{}{}", CFG.static_dir, path)).await
            }
        }
        _ => Ok(not_found()),
    }
}

async fn file_sender(path: String) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.
    if let Ok(file) = File::open(path).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        debug!("read success");
        return Ok(Response::new(body));
    }
    debug!("read fail");
    Ok(not_found())
}

// api server
async fn api_get(query: &str,segments:Vec<&str>) -> ApiRes {
    if query == "" || segments.len()<2 {
        return ApiRes::Null;
    }
    if check_query_token(&CFG.api_token,query) {
        match segments[1]{
            "info"=>{
                let game_servers = &CFG.game_servers;
                let servers_status = get_terraria_info(game_servers).await;
                info!("info:{:?}",servers_status);
                ApiRes::Success(serde_json::to_string(&servers_status).unwrap())
            },
            _=>{
                ApiRes::Null
            }
        }
        
    } else {
        ApiRes::TokenFail
    }
}

enum ApiRes {
    Success(String),
    TokenFail,
    Null,
}

fn check_query_token(token:&str,query: &str) -> bool {
    let params = url::form_urlencoded::parse(query.as_bytes());
    for (key, value) in params {
        if key == "token" && value == token {
            debug!("get info token success");
            return true;
        }
    }
    false
}
