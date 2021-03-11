mod config;
mod event_receiver;
mod websocket;

use std::net::Ipv4Addr;

use anyhow::Result;
use config::Config;
use event_receiver::PianobarEventReceiver;
use log::info;
use structopt::StructOpt;
use warp::Filter;
use websocket::PianobarWebsocket;

#[tokio::main]
async fn main() -> Result<()> {
    // Read configuration from command line arguments
    let config = Config::from_args();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        match config.verbose {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        },
    ))
    .init();

    info!("Create event handler ...");
    let event_receiver = PianobarEventReceiver::new(&config);

    info!("Create websocket ...");
    let websocket = PianobarWebsocket::new(event_receiver.get_event_source_creator());

    // Create Websocket route
    let websocket_route = websocket.create_route("ws");

    // Create web app route to serve static web app files if nothing else matches
    let webpage_route = config
        .webpage_folder
        .clone()
        .map(|webpage_folder| warp::get().and(warp::fs::dir(webpage_folder)));

    // Create the webserver task
    let webserver_task = async move {
        let addr = (Ipv4Addr::UNSPECIFIED, config.port);
        if let Some(webpage_route) = webpage_route {
            log::debug!("Serve websocket and webpage ...");
            warp::serve(websocket_route.or(webpage_route))
                .run(addr)
                .await;
        } else {
            log::debug!("Serve websocket ...");
            warp::serve(websocket_route).run(addr).await;
        }
        Ok(())
    };

    info!("Starting tasks ...");
    tokio::try_join!(webserver_task, event_receiver.run())?;

    info!("Program ended.");
    Ok(())
}
