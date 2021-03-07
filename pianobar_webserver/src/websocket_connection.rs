use crate::event_receiver::PianobarUiEvent;
use crate::event_receiver::PianobarUiEventSource;

use anyhow::{self, bail, Result};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use jsonrpc_core as jsonrpc;
use serde_json as json;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

// use futures::FutureExt;

pub struct PianobarWebsocketConnection {
    client_address: String,
    ui_events: PianobarUiEventSource,
}

impl PianobarWebsocketConnection {
    pub fn new(
        client_address: Option<SocketAddr>,
        ui_events: PianobarUiEventSource,
    ) -> PianobarWebsocketConnection {
        PianobarWebsocketConnection {
            client_address: match client_address {
                Some(s) => s.to_string(),
                None => "<UNKNOWN>".into(),
            },
            ui_events,
        }
    }

    pub async fn run(self, websocket: WebSocket) {
        let client_address = self.client_address.clone();
        log::info!("connected: {}", client_address);
        if let Err(err) = self.run_with_error_handling(websocket).await {
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
                        log::info!("closed with code {}", code);
                    } else {
                        log::info!("closed with code {}: {}", code, message);
                    }
                } else {
                    log::info!("closed without code");
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
        log::info!("send task ended");
    }

    async fn send_welcome_message(
        &self,
        send_queue: &mpsc::UnboundedSender<Message>,
    ) -> Result<()> {
        let welcome_message = jsonrpc::Notification {
            jsonrpc: Some(jsonrpc::Version::V2),
            method: "ui_event".to_string(),
            params: jsonrpc::Params::Map(
                PianobarUiEvent {
                    command: "websocket_welcome".to_string(),
                    state: self.ui_events.ui_initial_state.clone(),
                }
                .into(),
            ),
        };

        let message = json::to_string(&welcome_message)?;
        send_queue.send(Message::text(message))?;
        Ok(())
    }

    async fn run_with_error_handling(self, websocket: WebSocket) -> Result<()> {
        let (websocket_sender, websocket_receiver) = websocket.split();

        // Move sender to separate task and wrap in queue
        let (send_queue, send_queue_receiver) = mpsc::unbounded_channel::<warp::ws::Message>();
        tokio::task::spawn(PianobarWebsocketConnection::send_task(
            send_queue_receiver,
            websocket_sender,
        ));

        // Send welcome message
        log::info!("send welcome message ...");
        self.send_welcome_message(&send_queue).await?;

        // Start receive task
        let receive_task = self.receive_task(websocket_receiver, &send_queue);

        // Start event tasks

        // Wait until the first task finished
        tokio::select!(ret = receive_task=>ret)
    }
}
