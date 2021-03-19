use crate::event_receiver::{PianobarUiEvent, PianobarUiEventSource, PianobarUiState};
use crate::PianobarActions;

use super::json_rpc::JsonRpcWebsocket;
use super::pianobar_actions;
use super::PianobarPlayerState;
use anyhow::{self, Result};
use jsonrpc_core as jsonrpc;
use std::borrow::Borrow;
use std::net::SocketAddr;
use tokio::sync::{broadcast, watch};
use warp::ws::WebSocket;

// use futures::FutureExt;

impl jsonrpc::Metadata for PianobarActions {}

pub struct PianobarWebsocketConnection {
    client_address: String,
    json_rpc_websocket: JsonRpcWebsocket<PianobarActions>,
}

impl PianobarWebsocketConnection {
    pub fn new(
        client_address: Option<SocketAddr>,
        websocket: WebSocket,
    ) -> PianobarWebsocketConnection {
        PianobarWebsocketConnection {
            client_address: match client_address {
                Some(s) => s.to_string(),
                None => "<UNKNOWN>".to_string(),
            },
            json_rpc_websocket: JsonRpcWebsocket::new(websocket),
        }
    }

    pub async fn run(
        self,
        ui_events: PianobarUiEventSource,
        player_state: watch::Receiver<PianobarPlayerState>,
        pianobar_actions: PianobarActions,
    ) {
        let client_address = self.client_address.clone();
        log::info!("connected: {}", client_address);
        if let Err(err) = self
            .run_with_error_handling(ui_events, player_state, pianobar_actions)
            .await
        {
            log::warn!("lost connection: {}", err);
        }
        log::info!("disconnected: {}", client_address);
    }

    fn send_ui_event(&self, event: PianobarUiEvent) -> Result<()> {
        self.json_rpc_websocket
            .send_notification("ui_event", jsonrpc::Params::Map(event.into()))
    }

    fn send_player_state(&self, player_state: &PianobarPlayerState) -> Result<()> {
        let mut params = serde_json::Map::new();
        params.insert("state".to_string(), serde_json::to_value(player_state)?);

        self.json_rpc_websocket
            .send_notification("player_state", jsonrpc::Params::Map(params))
    }

    fn send_welcome_message(&self, ui_initial_state: PianobarUiState) -> Result<()> {
        self.send_ui_event(PianobarUiEvent {
            command: "websocket_welcome".to_string(),
            state: ui_initial_state,
        })
    }

    async fn events_task(&self, mut ui_events: broadcast::Receiver<PianobarUiEvent>) -> Result<()> {
        loop {
            let ui_event = ui_events.recv().await?;
            log::debug!("send ui event ...");
            self.send_ui_event(ui_event)?;
        }
    }

    async fn player_state_task(
        &self,
        mut player_state: watch::Receiver<PianobarPlayerState>,
    ) -> Result<()> {
        loop {
            player_state.changed().await?;
            log::debug!("send new player state ...");
            self.send_player_state(player_state.borrow().borrow())?;
        }
    }

    async fn run_with_error_handling(
        mut self,
        ui_events: PianobarUiEventSource,
        player_state: watch::Receiver<PianobarPlayerState>,
        pianobar_actions: PianobarActions,
    ) -> Result<()> {
        // Send welcome message
        log::debug!("send welcome message ...");
        self.send_welcome_message(ui_events.ui_initial_state)?;

        pianobar_actions::register(&mut self.json_rpc_websocket);

        // Start tasks
        let events_task = self.events_task(ui_events.ui_events);
        let player_state_task = self.player_state_task(player_state);

        // Wait until the first task finished
        tokio::select!(
            ret = self.json_rpc_websocket.run(pianobar_actions) => ret,
            ret = events_task => ret,
            ret = player_state_task => ret,
        )
    }
}
