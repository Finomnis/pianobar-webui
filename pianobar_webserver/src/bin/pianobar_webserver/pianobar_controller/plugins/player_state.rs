use super::{PianobarController, PianobarMessage};
use anyhow::{bail, Result};
use serde::Serialize;
use tokio::sync::{broadcast, watch};

#[derive(Clone, Debug, Serialize)]
pub struct PianobarPlayerState {
    pub song_time_played: u32,
    pub song_time_total: u32,
    pub paused: bool,
}

impl PianobarPlayerState {
    fn initial_state() -> Self {
        PianobarPlayerState {
            song_time_played: 0,
            song_time_total: 0,
            paused: true,
        }
    }
}

pub struct PianobarPlayerStateWatcher {
    receiver: broadcast::Receiver<PianobarMessage>,
    channel_in: watch::Sender<PianobarPlayerState>,
    channel_out: watch::Receiver<PianobarPlayerState>,
}

impl PianobarPlayerStateWatcher {
    pub fn new(controller: &PianobarController) -> Self {
        let (channel_in, channel_out) = watch::channel(PianobarPlayerState::initial_state());
        PianobarPlayerStateWatcher {
            receiver: controller.subscribe(),
            channel_in,
            channel_out,
        }
    }

    async fn process_message(&mut self, message: PianobarMessage) -> Result<()> {
        let mut state = self.channel_in.borrow().clone();

        match message {
            PianobarMessage::SongTime {
                current,
                total,
                paused,
            } => {
                state.song_time_played = current;
                state.song_time_total = total;
                state.paused = paused;
            } //_ => return Ok(()),
        };

        self.channel_in.send(state)?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let message = match self.receiver.recv().await {
                Ok(msg) => msg,
                Err(broadcast::error::RecvError::Lagged(num)) => {
                    log::warn!("Missed {} messages", num);
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => {
                    bail!("Pianobar internal stdout queue closed.")
                }
            };

            if let Err(err) = self.process_message(message).await {
                log::warn!("Unable to process message: {}", err);
            }
        }
    }

    pub fn subscribe(&self) -> watch::Receiver<PianobarPlayerState> {
        self.channel_out.clone()
    }
}
