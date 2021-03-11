use anyhow::{anyhow, bail, Result};
use log;
use std::process::Stdio;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{ChildStdin, ChildStdout, Command},
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

pub struct PianobarController {
    pianobar_command: String,
}

impl PianobarController {
    pub fn new(pianobar_command: &str) -> PianobarController {
        PianobarController {
            pianobar_command: pianobar_command.to_string(),
        }
    }

    async fn process_stdout(
        &self,
        queue: UnboundedSender<String>,
        mut pianobar_stream: ChildStdout,
    ) -> Result<()> {
        loop {
            let mut output = [0u8; 100000];
            let num_read = pianobar_stream.read(&mut output).await?;
            if num_read == 0 {
                bail!("pianobar program closed!");
            }
            let msg = std::str::from_utf8(&output[..num_read])?.to_string();
            queue.send(msg)?;
        }
    }

    async fn process_stdin(
        &self,
        mut queue: UnboundedReceiver<String>,
        mut pianobar_stream: ChildStdin,
    ) -> Result<()> {
        loop {
            let msg = queue
                .recv()
                .await
                .ok_or(anyhow!("Pianobar internal stdin queue closed."))?;
            pianobar_stream.write(msg.as_bytes()).await?;
        }
    }

    async fn control_logic(&self, mut pianobar_stdout: UnboundedReceiver<String>) -> Result<()> {
        loop {
            loop {
                let message = pianobar_stdout
                    .recv()
                    .await
                    .ok_or(anyhow!("Pianobar internal stdout queue closed."))?;
                log::debug!("\n{}", message);
            }
        }
    }

    async fn stdin_forwarder(&self, pianobar_stdin: &UnboundedSender<String>) -> Result<()> {
        loop {
            let mut buffer = [0u8; 1000];
            let mut stdin = tokio::io::stdin();
            let num_read = stdin.read(&mut buffer).await?;
            if num_read == 0 {
                // TODO don't kill the server when stdin closes, might be normal for a server process
                bail!("stdin closed!");
            }
            let message = std::str::from_utf8(&buffer[..num_read])?.to_string();
            pianobar_stdin.send(message)?;
        }
    }

    pub async fn run(&self) -> Result<()> {
        log::info!("Start pianobar process ...");
        let pianobar_process = Command::new(&self.pianobar_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let pianobar_stdin_stream = pianobar_process
            .stdin
            .ok_or(anyhow!("Unable to get pianobar stdin."))?;
        let pianobar_stdout_stream = pianobar_process
            .stdout
            .ok_or(anyhow!("Unable to get pianobar stdin."))?;

        let (pianobar_stdout_sink, pianobar_stdout) = mpsc::unbounded_channel::<String>();
        let stdout_task = self.process_stdout(pianobar_stdout_sink, pianobar_stdout_stream);

        let (pianobar_stdin, pianobar_stdin_source) = mpsc::unbounded_channel::<String>();
        let stdin_task = self.process_stdin(pianobar_stdin_source, pianobar_stdin_stream);

        tokio::select!(
            e = self.control_logic(pianobar_stdout) => e,
            e = self.stdin_forwarder(&pianobar_stdin) => e,
            e = stdout_task => e,
            e = stdin_task => e,
        )
    }
}
