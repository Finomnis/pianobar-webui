use crate::event_receiver::{PianobarUiEvent, PianobarUiEventSource, PianobarUiState};

use anyhow::{self, bail, Result};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use jsonrpc_core as jsonrpc;
use serde_json as json;
use std::net::SocketAddr;
use tokio::sync::{broadcast, mpsc};
use warp::ws::{Message, WebSocket};

// use futures::FutureExt;

pub struct PianobarWebsocketConnection {
    client_address: String,
}

impl PianobarWebsocketConnection {
    pub fn new(client_address: Option<SocketAddr>) -> PianobarWebsocketConnection {
        PianobarWebsocketConnection {
            client_address: match client_address {
                Some(s) => s.to_string(),
                None => "<UNKNOWN>".into(),
            },
        }
    }

    pub async fn run(self, websocket: WebSocket, ui_events: PianobarUiEventSource) {
        let client_address = self.client_address.clone();
        log::info!("connected: {}", client_address);
        if let Err(err) = self.run_with_error_handling(websocket, ui_events).await {
            log::warn!("lost connection: {}", err);
        }
        log::info!("disconnected: {}", client_address);
    }

    async fn handle_message(
        &self,
        message: &str,
        send_queue: &mpsc::UnboundedSender<Message>,
    ) -> Result<()> {
        // Just echo all messages
        send_queue.send(Message::text(message))?;

        // Handled message successfully
        Ok(())
    }

    async fn receive_task(
        &self,
        mut socket: SplitStream<WebSocket>,
        send_queue: &mpsc::UnboundedSender<Message>,
    ) -> Result<()> {
        while let Some(value) = socket.next().await {
            let value = value?;

            // Handle close
            if value.is_close() {
                if let Some((code, message)) = value.close_frame() {
                    if message.is_empty() {
                        log::debug!("closed with code {}", code);
                    } else {
                        log::debug!("closed with code {}: {}", code, message);
                    }
                } else {
                    log::debug!("closed without code");
                }
                // Send will fail, but triggering the send command again
                // is necessary to enable proper websocket shutdown
                send_queue.send(value)?;
                return Ok(());
            }
            // Handle message
            else if value.is_text() {
                let message = match value.to_str() {
                    Ok(msg) => msg,
                    Err(()) => bail!("expected string, didn't receive string"),
                };
                self.handle_message(message, send_queue).await?;
            }
        }

        // If the loop ended, this indicates that the client disconnected without a 'Close' message.
        Err(anyhow::Error::msg(
            "connection closed without close message",
        ))
    }

    async fn send_task(
        mut send_queue_receiver: mpsc::UnboundedReceiver<Message>,
        mut websocket_sender: SplitSink<WebSocket, Message>,
    ) {
        while let Some(item) = send_queue_receiver.recv().await {
            let is_close = item.is_close();
            if let Err(err) = websocket_sender.send(item).await {
                if !is_close {
                    log::warn!("send failed: {}", err);
                }
            }
        }
        log::debug!("send task ended");
    }

    async fn send_ui_event(
        &self,
        event: PianobarUiEvent,
        send_queue: &mpsc::UnboundedSender<Message>,
    ) -> Result<()> {
        let message = jsonrpc::Notification {
            jsonrpc: Some(jsonrpc::Version::V2),
            method: "ui_event".to_string(),
            params: jsonrpc::Params::Map(event.into()),
        };

        send_queue.send(Message::text(json::to_string(&message)?))?;
        Ok(())
    }

    async fn send_welcome_message(
        &self,
        ui_initial_state: PianobarUiState,
        send_queue: &mpsc::UnboundedSender<Message>,
    ) -> Result<()> {
        self.send_ui_event(
            PianobarUiEvent {
                command: "websocket_welcome".to_string(),
                state: ui_initial_state,
            },
            send_queue,
        )
        .await
    }

    async fn events_task(
        &self,
        mut ui_events: broadcast::Receiver<PianobarUiEvent>,
        send_queue: &mpsc::UnboundedSender<Message>,
    ) -> Result<()> {
        loop {
            let ui_event = ui_events.recv().await?;
            log::debug!("send ui event ...");
            self.send_ui_event(ui_event, send_queue).await?;
        }
    }

    async fn run_with_error_handling(
        self,
        websocket: WebSocket,
        ui_events: PianobarUiEventSource,
    ) -> Result<()> {
        let (websocket_sender, websocket_receiver) = websocket.split();

        // Move sender to separate task and wrap in queue
        let (send_queue, send_queue_receiver) = mpsc::unbounded_channel::<warp::ws::Message>();
        tokio::task::spawn(PianobarWebsocketConnection::send_task(
            send_queue_receiver,
            websocket_sender,
        ));

        // Send welcome message
        log::debug!("send welcome message ...");
        self.send_welcome_message(ui_events.ui_initial_state, &send_queue)
            .await?;

        // Start receive task
        let receive_task = self.receive_task(websocket_receiver, &send_queue);

        // Start event tasks
        let events_task = self.events_task(ui_events.ui_events, &send_queue);

        // Wait until the first task finished
        tokio::select!(
            ret = receive_task => ret,
            ret = events_task => ret,
        )
    }
}
