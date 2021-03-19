use anyhow::{bail, Result};
use tokio::sync::broadcast;

use super::{PianobarController, PianobarMessage};

pub struct DebugPrinter {
    receiver: broadcast::Receiver<PianobarMessage>,
}

impl DebugPrinter {
    pub fn new(controller: &PianobarController) -> Self {
        DebugPrinter {
            receiver: controller.subscribe(),
        }
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

            log::debug!("Message: {:?}", message);
        }
    }
}
