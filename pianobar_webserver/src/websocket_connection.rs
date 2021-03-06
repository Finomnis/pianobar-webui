use crate::event_receiver::PianobarUiEvent;
use crate::event_receiver::PianobarUiEventSource;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use jsonrpc_core as jsonrpc;
use serde_json as json;
use std::net::SocketAddr;
use warp::ws::WebSocket;

pub struct PianobarWebsocketConnection {
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
        let client_address = self.client_address.clone();
        log::info!("connected: {}", client_address);
        if let Err(err) = self.run_with_error_handling().await {
            log::info!("lost connection: {}", err);
        }
        log::info!("disconnected: {}", client_address);
    }

    pub async fn run_with_error_handling(self) -> Result<()> {
        let (mut tx, rx) = self.websocket.split();
        log::info!("starting echo ...");

        // Send welcome message
        {
            let welcome_message = jsonrpc::Notification {
                jsonrpc: Some(jsonrpc::Version::V2),
                method: "ui_event".to_string(),
                params: jsonrpc::Params::Map(
                    PianobarUiEvent {
                        command: "websocket_welcome".to_string(),
                        state: self.ui_events.ui_initial_state,
                    }
                    .into(),
                ),
            };

            tx.send(warp::ws::Message::text(json::to_string(&welcome_message)?))
                .await?;
        }

        // Start communication
        rx.forward(tx).await?;

        Ok(())
    }
}
