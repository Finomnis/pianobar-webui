use super::PianobarController;
use super::{PianobarActor, PianobarMessage};
use anyhow::{anyhow, bail, Result};
use std::sync::{Arc, Weak};
use tokio::sync::broadcast;

struct PianobarActionsConnection {
    pianobar_controller: Arc<PianobarController>,
}

impl PianobarActionsConnection {
    async fn lock(
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
}

#[derive(Clone)]
pub struct PianobarActions {
    pianobar_controller: Weak<PianobarController>,
}

fn with_reset(msg: &str) -> String {
    format!("\r\n\r\n{}", msg)
}

impl PianobarActions {
    pub fn new(pianobar_controller: Weak<PianobarController>) -> PianobarActions {
        PianobarActions {
            pianobar_controller,
        }
    }

    fn connect(&self) -> Result<PianobarActionsConnection> {
        Ok(PianobarActionsConnection {
            pianobar_controller: self
                .pianobar_controller
                .upgrade()
                .ok_or(anyhow!("Unable to take pianobar actions object!"))?,
        })
    }

    async fn simple_command(&self, cmd: &str) -> Result<()> {
        let pianobar_actions_connection = self.connect()?;
        let (mut _receiver, mut actor) = pianobar_actions_connection.lock().await;

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
        let pianobar_actions_connection = self.connect()?;
        let (mut _receiver, mut actor) = pianobar_actions_connection.lock().await;

        actor.write(&with_reset("e")).await?;

        // TODO implement reading the actual result from the receiver
        bail!("NOT IMPLEMENTED YET".to_string());
    }
}
