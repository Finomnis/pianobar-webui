use anyhow::{anyhow, Result};
use log;
use std::process::Stdio;
use tokio::{io::AsyncReadExt, process::Command};

pub struct PianobarController {
    pianobar_command: String,
}

impl PianobarController {
    pub fn new(pianobar_command: &str) -> PianobarController {
        PianobarController {
            pianobar_command: pianobar_command.to_string(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        log::info!("Start pianobar process ...");
        let pianobar_process = Command::new(&self.pianobar_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let mut pianobar_in = pianobar_process
            .stdin
            .ok_or(anyhow!("Unable to get pianobar stdin."))?;
        let mut pianobar_out = pianobar_process
            .stdout
            .ok_or(anyhow!("Unable to get pianobar stdin."))?;

        loop {
            let mut output = [0u8; 300];
            let num_read = pianobar_out.read(&mut output).await?;
            log::debug!(
                "{:?}\n{}",
                &output[..num_read],
                std::str::from_utf8(&output[..num_read])?
            );
        }
    }
}
