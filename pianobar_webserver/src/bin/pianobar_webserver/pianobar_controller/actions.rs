use super::PianobarController;
use super::{PianobarActor, PianobarMessage};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct PianobarActions {
    pianobar_controller: Arc<PianobarController>,
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

    pub async fn change_station(&self, station_id: usize) -> Result<()> {
        let (mut _receiver, mut actor) = self.connect().await;

        actor.write(&format!("\r\ns{}\n", station_id)).await?;

        Ok(())
    }
}
