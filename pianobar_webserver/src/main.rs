mod config;
mod event_handler;
mod websocket;

use anyhow::Result;
use config::Config;
use event_handler::PianobarEventHandler;
use log::info;
use structopt::StructOpt;
use warp::Filter;
use websocket::PianobarWebsocket;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = Config::from_args();

    info!("Create websocket ...");
    let websocket = PianobarWebsocket::new();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let websocket_route = websocket.create_route("ws");
    // let websocket_route = warp::path!("ws")
    //     .and(warp::ws())
    //     .and(warp::addr::remote())
    //     .and_then(handler);

    let routes = hello
        .or(websocket_route)
        .with(warp::cors().allow_any_origin());

    let webserver_task = async move {
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
        Ok(())
    };

    info!("Create event handler ...");
    let event_handler = PianobarEventHandler::new(&config);
    let a = event_handler.get_ui_events();
    let b = event_handler.get_ui_state();

    info!("Starting tasks ...");
    tokio::try_join!(webserver_task, event_handler.run())?;

    info!("Program ended.");
    Ok(())
}
