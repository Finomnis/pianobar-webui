mod websocket;

use anyhow::Result;
use log::info;
use warp::Filter;
use websocket::handler;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    //info!("Create websocket object ...");
    //let websocket = Websocket::new();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let websocket_route = warp::path!("ws")
        .and(warp::ws())
        .and(warp::addr::remote())
        .and_then(handler);

    let routes = hello
        .or(websocket_route)
        .with(warp::cors().allow_any_origin());

    let webserver_task = warp::serve(routes).run(([127, 0, 0, 1], 3030));

    info!("Starting tasks ...");
    tokio::join!(webserver_task);

    info!("Program ended.");
    Ok(())
}
