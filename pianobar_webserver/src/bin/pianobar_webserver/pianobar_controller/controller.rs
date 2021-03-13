use anyhow::{anyhow, bail, Result};
use log;
use std::process::Stdio;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{ChildStdin, ChildStdout, Command},
    sync::broadcast,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

#[derive(Clone, Debug)]
enum PianobarMessage {
    UnknownMessage(String),
}

pub struct PianobarController {
    pianobar_command: String,
    pianobar_received_messages: broadcast::Sender<PianobarMessage>,
}

impl PianobarController {
    pub fn new(pianobar_command: &str) -> PianobarController {
        let (pianobar_received_messages, _) = broadcast::channel(20);
        PianobarController {
            pianobar_received_messages,
            pianobar_command: pianobar_command.to_string(),
        }
    }

    async fn process_stdout(&self, mut pianobar_stream: ChildStdout) -> Result<()> {
        loop {
            // TODO add custom message format to pianobar config,
            // parse stream into messages
            let mut output = [0u8; 100000];
            let num_read = pianobar_stream.read(&mut output).await?;
            if num_read == 0 {
                bail!("pianobar program closed!");
            }
            let msg = std::str::from_utf8(&output[..num_read])?.to_string();

            log::debug!("\n{}", msg);

            match self
                .pianobar_received_messages
                .send(PianobarMessage::UnknownMessage(msg))
            {
                Ok(num_receivers) => {
                    log::debug!("Sent pianobar message to {} listeners.", num_receivers)
                }
                Err(broadcast::error::SendError(msg)) => {
                    log::error!("No receiver for message: {:?}", msg);
                }
            };
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

            // Get slice to send
            let mut send_buffer = msg.as_bytes();
            while send_buffer.len() > 0 {
                let num_sent = pianobar_stream.write(send_buffer).await?;
                if num_sent == 0 {
                    bail!("Unable to write to pianobar process");
                }
                // Remove the sent bytes from the slice
                send_buffer = &send_buffer[num_sent..];
            }

            // Flush, to make sure messages without newlines get delivered
            pianobar_stream.flush().await?;
        }
    }

    async fn control_logic(&self) -> Result<()> {
        let mut receiver = self.pianobar_received_messages.subscribe();
        loop {
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

        let stdout_task = self.process_stdout(pianobar_stdout_stream);

        let (pianobar_stdin, pianobar_stdin_source) = mpsc::unbounded_channel::<String>();
        let stdin_task = self.process_stdin(pianobar_stdin_source, pianobar_stdin_stream);

        tokio::try_join!(
            self.control_logic(),
            self.stdin_forwarder(&pianobar_stdin),
            stdout_task,
            stdin_task,
        )?;

        bail!("All controller tasks ended unexpectedly.");
    }
}
