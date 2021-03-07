use crate::event_receiver::{PianobarUiEventSource, PianobarUiEventSourceCreator};

use crate::websocket_connection::PianobarWebsocketConnection;

use std::net::SocketAddr;
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
            let client = PianobarWebsocketConnection::new(addr, ui_events);
            client.run(socket)
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
