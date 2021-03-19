use anyhow::Result;
use tokio::io::AsyncReadExt;

use super::super::PianobarController;

pub struct ManualController {
    controller: PianobarController,
}

impl ManualController {
    pub fn new(controller: &PianobarController) -> Self {
        Self {
            controller: controller.clone(),
        }
    }
    pub async fn run(&mut self) -> Result<()> {
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
            self.controller.take_actor().await.write(&message).await?;
        }
    }
}
