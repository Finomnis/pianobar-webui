use anyhow::Result;
use tokio::io::AsyncReadExt;

use super::super::PianobarController;

pub async fn manual_controller(controller: &PianobarController) -> Result<()> {
    loop {
        let mut buffer = [0u8; 100];
        let mut stdin = tokio::io::stdin();
        let num_read = stdin.read(&mut buffer).await?;
        if num_read == 0 {
            // Don't kill the server when stdin closes, might be normal for a server process
            log::debug!("Stdin closed.");
            return Ok(());
        }
        let message = std::str::from_utf8(&buffer[..num_read])?.to_string();
        controller.take_actor().await.write(&message).await?;
    }
}
