use anyhow::{bail, Result};
use tokio::sync::broadcast;

use super::super::PianobarController;

pub async fn debug_printer(controller: &PianobarController) -> Result<()> {
    let mut receiver = controller.subscribe();

    loop {
        let message = match receiver.recv().await {
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
