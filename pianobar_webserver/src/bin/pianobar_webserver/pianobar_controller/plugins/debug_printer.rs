use anyhow::{bail, Result};
use tokio::sync::broadcast;

use super::super::PianobarController;

pub async fn debug_printer(controller: &PianobarController) -> Result<()> {
    let mut receiver = controller.subscribe();

    loop {
        // TODO process remote procedure calls somehow
        // - don't process them from a queue, instead let them operate on
        // the server directly. Use a mutex for synchronization.
        // Might want to use the receive stream mutex directly.
        // (that way the thread can be killed without blocking everything)
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
