use anyhow::{self, bail, Result};
use futures::stream::SplitStream;
use futures::{SinkExt, StreamExt};
use jsonrpc_core as jsonrpc;
use serde_json as json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};

pub struct JsonRpcWebsocket {
    send_queue: mpsc::UnboundedSender<Message>,
    websocket_receiver: Arc<Mutex<SplitStream<WebSocket>>>,
}

impl JsonRpcWebsocket {
    pub fn new(websocket: WebSocket) -> JsonRpcWebsocket {
        let (mut websocket_sender, websocket_receiver) = websocket.split();

        // Move sender to separate task and abstract it behind an mpsc queue
        let (send_queue, mut send_queue_receiver) = mpsc::unbounded_channel::<warp::ws::Message>();
        tokio::task::spawn(async move {
            while let Some(item) = send_queue_receiver.recv().await {
                let is_close = item.is_close();
                if let Err(err) = websocket_sender.send(item).await {
                    if !is_close {
                        log::warn!("send failed: {}", err);
                    }
                    break;
                }
            }
            log::debug!("send task ended");
        });

        JsonRpcWebsocket {
            send_queue,
            websocket_receiver: Arc::new(Mutex::new(websocket_receiver)),
        }
    }

    pub fn send_notification(&self, method: &str, params: jsonrpc::Params) -> Result<()> {
        let message = jsonrpc::Notification {
            jsonrpc: Some(jsonrpc::Version::V2),
            method: method.to_string(),
            params: params,
        };

        self.send_queue
            .send(Message::text(json::to_string(&message)?))?;

        Ok(())
    }

    /// Processes a text message sent by the connected user
    ///
    /// * `message` - The content of the message
    /// * `send_queue` - The queue object used to send responses
    ///
    async fn handle_message(&self, message: &str) -> Result<()> {
        // Just echo all messages
        self.send_queue.send(Message::text(message))?;

        // Handled message successfully
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let mut websocket_receiver = self.websocket_receiver.try_lock()?;

        while let Some(value) = websocket_receiver.next().await {
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
                self.send_queue.send(value)?;
                return Ok(());
            }
            // Handle message
            else if value.is_text() {
                let message = match value.to_str() {
                    Ok(msg) => msg,
                    Err(()) => bail!("expected string, didn't receive string"),
                };
                self.handle_message(message).await?;
            }
        }

        // If the loop ended, this indicates that the client disconnected without a 'Close' message.
        Err(anyhow::Error::msg(
            "connection closed without close message",
        ))
    }
}
