use std::time::Duration;

use super::PianobarController;
use super::{PianobarActor, PianobarMessage};
use anyhow::Result;
use serde::Serialize;
use tokio::{sync::broadcast, time::timeout};

#[derive(Clone)]
pub struct PianobarActions {
    pianobar_controller: PianobarController,
}

fn with_reset(msg: &str) -> String {
    format!("\r\n\r\n{}", msg)
}

#[derive(Serialize)]
pub struct HistoryEntry {
    artist: String,
    title: String,
}

impl PianobarActions {
    pub fn new(pianobar_controller: &PianobarController) -> PianobarActions {
        PianobarActions {
            pianobar_controller: pianobar_controller.clone(),
        }
    }

    async fn lock(
        &self,
    ) -> (
        broadcast::Receiver<PianobarMessage>,
        tokio::sync::MutexGuard<'_, PianobarActor>,
    ) {
        // First lock the actor, then get the receiver.
        // This synchronizes the receiver as well and might prevent race conditions.
        let actor = self.pianobar_controller.take_actor().await;
        let receiver = self.pianobar_controller.subscribe();
        (receiver, actor)
    }

    async fn simple_command(&self, cmd: &str) -> Result<()> {
        let (mut _receiver, mut actor) = self.lock().await;

        actor.write(&with_reset(cmd)).await?;

        Ok(())
    }

    pub async fn change_station(&self, station_id: usize) -> Result<()> {
        log::info!("Changing station to #{} ...", station_id);
        self.simple_command(&format!("s{}\n", station_id)).await
    }

    pub async fn pause(&self) -> Result<()> {
        log::info!("Pausing ...");
        self.simple_command("S").await
    }

    pub async fn resume(&self) -> Result<()> {
        log::info!("Resuming ...");
        self.simple_command("P").await
    }

    pub async fn toggle_pause(&self) -> Result<()> {
        log::info!("Toggling pause ...");
        self.simple_command("p").await
    }

    pub async fn skip(&self) -> Result<()> {
        log::info!("Skipping ...");
        self.simple_command("n").await
    }

    pub async fn explain(&self) -> Result<String> {
        log::info!("Explaining ...");
        let (receiver, mut actor) = self.lock().await;

        actor.write(&with_reset("e")).await?;

        async fn read_response(
            mut receiver: broadcast::Receiver<PianobarMessage>,
        ) -> anyhow::Result<String> {
            loop {
                if let PianobarMessage::Info { message } = receiver.recv().await? {
                    if message.starts_with("We're playing this track because") {
                        return Ok(message);
                    }
                }
            }
        };

        Ok(timeout(Duration::from_millis(1000), read_response(receiver)).await??)
    }

    pub async fn history(&self) -> Result<Vec<HistoryEntry>> {
        log::info!("Retreiving history ...");
        let (receiver, mut actor) = self.lock().await;

        actor.write(&with_reset("h\r\n\r\n")).await?;

        async fn read_response(
            mut receiver: broadcast::Receiver<PianobarMessage>,
        ) -> anyhow::Result<Vec<HistoryEntry>> {
            let mut entries = vec![];
            loop {
                let msg = receiver.recv().await?;
                match msg {
                    PianobarMessage::Info { message } => {
                        if message == "No history yet." {
                            return Ok(entries);
                        }
                    }
                    PianobarMessage::Question { message: _ } => return Ok(entries),
                    PianobarMessage::ListEntrySong { artist, title } => {
                        entries.push(HistoryEntry { artist, title });
                    }
                    _ => {}
                };
            }
        };

        Ok(timeout(Duration::from_millis(1000), read_response(receiver)).await??)
    }
}
