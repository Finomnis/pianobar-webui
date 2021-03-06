use crate::config::Config;
use anyhow::Result;
use serde::Deserialize;
use serde_json as json;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, watch};

#[derive(Debug, Deserialize, Clone)]
pub struct PianobarUiEvent {
    pub command: String,
    pub state: PianobarUiState,
}

pub type PianobarUiState = HashMap<String, json::Value>;

#[derive(Clone)]
pub struct PianobarUiEventSourceCreator {
    ui_state: watch::Receiver<PianobarUiState>,
    ui_events: broadcast::Sender<PianobarUiEvent>,
}

pub struct PianobarUiEventSource {
    pub ui_initial_state: PianobarUiState,
    pub ui_events: broadcast::Receiver<PianobarUiEvent>,
}

impl PianobarUiEventSourceCreator {
    pub fn create_event_source(&self) -> PianobarUiEventSource {
        PianobarUiEventSource {
            ui_initial_state: self.ui_state.borrow().clone(),
            ui_events: self.ui_events.subscribe(),
        }
    }
}

pub struct PianobarEventReceiver {
    port: u16,
    ui_state: watch::Receiver<PianobarUiState>,
    update_ui_state: watch::Sender<PianobarUiState>,
    ui_events: broadcast::Sender<PianobarUiEvent>,
    _ui_events_dummy_receiver: broadcast::Receiver<PianobarUiEvent>,
}

impl PianobarEventReceiver {
    pub fn new(config: &Config) -> PianobarEventReceiver {
        let (update_ui_state, ui_state) = watch::channel(PianobarUiState::new());
        let (ui_events, _ui_events_dummy_receiver) = broadcast::channel(10);
        PianobarEventReceiver {
            port: config.event_port,
            update_ui_state,
            ui_state,
            ui_events,
            _ui_events_dummy_receiver,
        }
    }

    pub fn get_event_source_creator(&self) -> PianobarUiEventSourceCreator {
        PianobarUiEventSourceCreator {
            ui_state: self.ui_state.clone(),
            ui_events: self.ui_events.clone(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        log::info!("Start event handler ...");
        let listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), self.port)).await?;
        log::info!("Listening on port {}.", self.port);

        loop {
            let (mut socket, addr) = listener.accept().await?;

            let message = {
                let mut buf = vec![];

                // In a loop, read data from the socket and write the data back.
                loop {
                    match socket.read_buf(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => break Ok(buf),
                        Ok(_) => (),
                        Err(e) => break Err(e),
                    };
                }
            };

            let message = match message {
                Ok(msg) => msg,
                Err(err) => {
                    log::warn!("Error while receiving message: {}", err);
                    continue;
                }
            };

            log::info!("Event received from {}", addr);
            let event = match json::from_slice::<PianobarUiEvent>(&message) {
                Ok(a) => a,
                Err(err) => {
                    log::warn!("Error while decoding json: {}", err);
                    continue;
                }
            };

            if let Err(err) = self.update_ui_state.send(event.state.clone()) {
                log::error!("Error while updating ui state: {}", err);
            };

            if let Err(err) = self.ui_events.send(event) {
                log::warn!("Error while broadcasting ui event: {}", err);
            };
        }
    }
}
