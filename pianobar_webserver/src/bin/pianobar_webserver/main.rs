mod config;
mod event_receiver;
mod pianobar_controller;
mod signal_handler;
mod websocket;

use std::net::Ipv4Addr;

use anyhow::Result;
use config::Config;
use event_receiver::PianobarEventReceiver;
use log::info;
use pianobar_controller::PianobarController;
use signal_handler::handle_interrupt_signals;
use structopt::StructOpt;
use warp::Filter;
use websocket::PianobarWebsocket;

#[tokio::main]
async fn main() {
    // Call exit(), because just ending main doesn't kill open threads
    match main_with_result().await {
        Ok(()) => {
            log::info!("Program ended.");
            std::process::exit(0);
        }
        Err(err) => {
            log::error!("Program ended with error: {}", err);
            std::process::exit(1);
        }
    }
}

async fn main_with_result() -> Result<()> {
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

    info!("Create pianobar controller ...");
    let pianobar_controller = PianobarController::new(&config.pianobar_path);

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
    tokio::select!(
        e = webserver_task => e,
        e = event_receiver.run() => e,
        e = pianobar_controller.run() => e,
        e = handle_interrupt_signals() => e,
    )
}
