use futures::StreamExt;
use log;
use std::net::SocketAddr;
use warp::ws::WebSocket;
use warp::{Filter, Rejection, Reply};

pub struct PianobarWebsocket {}

impl PianobarWebsocket {
    pub fn new() -> PianobarWebsocket {
        PianobarWebsocket {}
    }

    async fn connection_handler(
        ws: warp::ws::Ws,
        addr: Option<SocketAddr>,
    ) -> std::result::Result<impl Reply, Rejection> {
        Ok(ws.on_upgrade(move |socket| client_connection(socket, addr)))
    }

    pub fn create_route(
        &self,
        path: &'static str,
    ) -> impl warp::Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        warp::path(path)
            .and(warp::ws())
            .and(warp::addr::remote())
            .and_then(PianobarWebsocket::connection_handler)
    }
}

pub async fn client_connection(websocket: WebSocket, addr_raw: Option<SocketAddr>) {
    let addr = match addr_raw {
        Some(s) => s.to_string(),
        None => "<UNKNOWN>".into(),
    };
    log::info!("connected: {}", addr);
    let (tx, rx) = websocket.split();

    log::info!("starting echo ...");
    if let Err(err) = rx.forward(tx).await {
        match err {
            _ => log::info!("lost connection: {}", err),
        };
    }
    log::info!("disconnected: {}", addr);
}

pub async fn handler(
    ws: warp::ws::Ws,
    addr: Option<SocketAddr>,
) -> std::result::Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| client_connection(socket, addr)))
}
