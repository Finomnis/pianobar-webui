use super::PianobarController;
use super::{PianobarActor, PianobarMessage};
use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct PianobarActions {
    pianobar_controller: Arc<PianobarController>,
}

fn with_reset(msg: &str) -> String {
    format!("\r\n\r\n{}", msg)
}

impl PianobarActions {
    pub fn new(pianobar_controller: Arc<PianobarController>) -> PianobarActions {
        PianobarActions {
            pianobar_controller,
        }
    }

    async fn connect(
        &self,
    ) -> (
        broadcast::Receiver<PianobarMessage>,
        tokio::sync::MutexGuard<'_, PianobarActor>,
    ) {
        (
            self.pianobar_controller.subscribe(),
            self.pianobar_controller.take_actor().await,
        )
    }

    async fn simple_command(&self, cmd: &str) -> Result<()> {
        let (mut _receiver, mut actor) = self.connect().await;

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
        let (mut _receiver, mut actor) = self.connect().await;

        actor.write(&with_reset("e")).await?;

        // TODO implement reading the actual result from the receiver
        bail!("NOT IMPLEMENTED YET".to_string());
    }
}
