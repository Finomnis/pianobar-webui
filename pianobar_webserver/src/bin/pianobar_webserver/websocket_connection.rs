use crate::event_receiver::{PianobarUiEvent, PianobarUiEventSource, PianobarUiState};

use crate::websocket_json_rpc::JsonRpcWebsocket;
use anyhow::{self, Result};
use jsonrpc_core as jsonrpc;
use std::net::SocketAddr;
use tokio::sync::broadcast;
use warp::ws::WebSocket;

// use futures::FutureExt;

pub struct PianobarWebsocketConnection {
    client_address: String,
    json_rpc_websocket: JsonRpcWebsocket,
}

impl PianobarWebsocketConnection {
    pub fn new(
        client_address: Option<SocketAddr>,
        websocket: WebSocket,
    ) -> PianobarWebsocketConnection {
        PianobarWebsocketConnection {
            client_address: match client_address {
                Some(s) => s.to_string(),
                None => "<UNKNOWN>".into(),
            },
            json_rpc_websocket: JsonRpcWebsocket::new(websocket),
        }
    }

    pub async fn run(self, ui_events: PianobarUiEventSource) {
        let client_address = self.client_address.clone();
        log::info!("connected: {}", client_address);
        if let Err(err) = self.run_with_error_handling(ui_events).await {
            log::warn!("lost connection: {}", err);
        }
        log::info!("disconnected: {}", client_address);
    }

    fn send_ui_event(&self, event: PianobarUiEvent) -> Result<()> {
        self.json_rpc_websocket
            .send_notification("ui_event", jsonrpc::Params::Map(event.into()))
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

    async fn run_with_error_handling(self, ui_events: PianobarUiEventSource) -> Result<()> {
        // Send welcome message
        log::debug!("send welcome message ...");
        self.send_welcome_message(ui_events.ui_initial_state)?;

        // Start event tasks
        let events_task = self.events_task(ui_events.ui_events);

        // Wait until the first task finished
        tokio::select!(
            ret = self.json_rpc_websocket.run() => ret,
            ret = events_task => ret,
        )
    }
}
