use crate::event_receiver::{PianobarUiEventSource, PianobarUiEventSourceCreator};
use crate::PianobarActions;

use super::connection::PianobarWebsocketConnection;
use super::PianobarPlayerState;

use std::net::SocketAddr;
use tokio::sync::watch;
use warp::{Filter, Rejection, Reply};

pub struct PianobarWebsocket {
    pianobar_ui_event_source_creator: PianobarUiEventSourceCreator,
    pianobar_player_state: watch::Receiver<PianobarPlayerState>,
    pianobar_actions: PianobarActions,
}

impl PianobarWebsocket {
    pub fn new(
        pianobar_ui_event_source_creator: PianobarUiEventSourceCreator,
        pianobar_player_state: watch::Receiver<PianobarPlayerState>,
        pianobar_actions: PianobarActions,
    ) -> PianobarWebsocket {
        PianobarWebsocket {
            pianobar_ui_event_source_creator,
            pianobar_player_state,
            pianobar_actions,
        }
    }

    async fn connection_upgrader(
        ws: warp::ws::Ws,
        addr: Option<SocketAddr>,
        ui_events: PianobarUiEventSource,
        player_state: watch::Receiver<PianobarPlayerState>,
        pianobar_actions: PianobarActions,
    ) -> std::result::Result<impl Reply, Rejection> {
        Ok(ws.on_upgrade(move |socket| {
            let client = PianobarWebsocketConnection::new(addr, socket);
            client.run(ui_events, player_state, pianobar_actions)
        }))
    }

    pub fn create_route(
        &self,
        path: &'static str,
    ) -> impl warp::Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        warp::path(path)
            .and(warp::ws())
            .and(warp::addr::remote())
            .and(self.with_ui_events())
            .and(self.with_player_state())
            .and(self.with_pianobar_actions())
            .and_then(PianobarWebsocket::connection_upgrader)
    }

    fn with_ui_events(
        &self,
    ) -> impl Filter<Extract = (PianobarUiEventSource,), Error = std::convert::Infallible> + Clone
    {
        let source_creator = self.pianobar_ui_event_source_creator.clone();
        warp::any().map(move || source_creator.create_event_source())
    }

    fn with_player_state(
        &self,
    ) -> impl Filter<
        Extract = (watch::Receiver<PianobarPlayerState>,),
        Error = std::convert::Infallible,
    > + Clone {
        let pianobar_player_state = self.pianobar_player_state.clone();
        warp::any().map(move || pianobar_player_state.clone())
    }

    fn with_pianobar_actions(
        &self,
    ) -> impl Filter<Extract = (PianobarActions,), Error = std::convert::Infallible> + Clone {
        let source_creator = self.pianobar_actions.clone();
        warp::any().map(move || source_creator.clone())
    }
}
