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
            // TODO add custom message format to pianobar config,
            // parse stream into messages
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
                // TODO process remote procedure calls somehow
                // - don't process them from a queue, instead let them operate on
                // the server directly. Use a mutex for synchronization.
                // Might want to use the receive stream mutex directly.
                // (that way the thread can be killed without blocking everything)
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
                // Don't kill the server when stdin closes, might be normal for a server process
                log::debug!("Stdin closed.");
                return Ok(());
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
            .ok_or(anyhow!("Unable to get pianobar stdout."))?;

        let (pianobar_stdout_sink, pianobar_stdout) = mpsc::unbounded_channel::<String>();
        let stdout_task = self.process_stdout(pianobar_stdout_sink, pianobar_stdout_stream);

        let (pianobar_stdin, pianobar_stdin_source) = mpsc::unbounded_channel::<String>();
        let stdin_task = self.process_stdin(pianobar_stdin_source, pianobar_stdin_stream);

        tokio::try_join!(
            self.control_logic(pianobar_stdout),
            self.stdin_forwarder(&pianobar_stdin),
            stdout_task,
            stdin_task,
        )?;

        bail!("All controller tasks ended unexpectedly.");
    }
}
