use crate::event_receiver::{PianobarUiEventSource, PianobarUiEventSourceCreator};
use futures::StreamExt;
use log;
use std::net::SocketAddr;
use warp::ws::WebSocket;
use warp::{Filter, Rejection, Reply};

pub struct PianobarWebsocket {
    pianobar_ui_event_source_creator: PianobarUiEventSourceCreator,
}

impl PianobarWebsocket {
    pub fn new(
        pianobar_ui_event_source_creator: PianobarUiEventSourceCreator,
    ) -> PianobarWebsocket {
        PianobarWebsocket {
            pianobar_ui_event_source_creator,
        }
    }

    async fn connection_upgrader(
        ws: warp::ws::Ws,
        addr: Option<SocketAddr>,
        ui_events: PianobarUiEventSource,
    ) -> std::result::Result<impl Reply, Rejection> {
        Ok(ws.on_upgrade(move |socket| {
            let client = PianobarWebsocketConnection::new(socket, addr, ui_events);
            client.run()
        }))
    }

    pub fn create_route(
        &self,
        path: &'static str,
    ) -> impl warp::Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        warp::path(path)
            .and(warp::ws())
            .and(warp::addr::remote())
            .and(self.with_ui_event_source())
            .and_then(PianobarWebsocket::connection_upgrader)
    }

    fn with_ui_event_source(
        &self,
    ) -> impl Filter<Extract = (PianobarUiEventSource,), Error = std::convert::Infallible> + Clone
    {
        let source_creator = self.pianobar_ui_event_source_creator.clone();
        warp::any().map(move || source_creator.create_event_source())
    }
}

struct PianobarWebsocketConnection {
    websocket: WebSocket,
    client_address: String,
    ui_events: PianobarUiEventSource,
}

impl PianobarWebsocketConnection {
    pub fn new(
        websocket: WebSocket,
        client_address: Option<SocketAddr>,
        ui_events: PianobarUiEventSource,
    ) -> PianobarWebsocketConnection {
        PianobarWebsocketConnection {
            websocket,
            client_address: match client_address {
                Some(s) => s.to_string(),
                None => "<UNKNOWN>".into(),
            },
            ui_events,
        }
    }

    pub async fn run(self) {
        log::info!("connected: {}", self.client_address);
        let (tx, rx) = self.websocket.split();
        log::info!("starting echo ...");
        if let Err(err) = rx.forward(tx).await {
            match err {
                _ => log::info!("lost connection: {}", err),
            };
        }
        log::info!("disconnected: {}", self.client_address);
    }
}
