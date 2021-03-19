use super::messages::{parse_pianobar_messages, PianobarMessage};

use anyhow::{anyhow, bail, Result};
use log;
use std::{process::Stdio, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::broadcast,
    sync::Mutex,
};

/// Provides an interface that can be used by function calls to send
/// commands to the pianobar process.
///
/// Solved as a separate struct instead of an mpsc, because it's
/// important that only one person can communicate with the process
/// at any given time. This is ensured by wrapping this struct in a
/// mutex.
pub struct PianobarActor {
    pianobar_stdin: ChildStdin,
}

impl PianobarActor {
    pub fn new(pianobar_stdin: ChildStdin) -> PianobarActor {
        PianobarActor { pianobar_stdin }
    }

    pub async fn write(&mut self, message: &str) -> Result<()> {
        // Get slice to send
        let mut send_buffer = message.as_bytes();
        while send_buffer.len() > 0 {
            let num_sent = self.pianobar_stdin.write(send_buffer).await?;
            if num_sent == 0 {
                bail!("Unable to write to pianobar process");
            }
            // Remove the sent bytes from the slice
            send_buffer = &send_buffer[num_sent..];
        }

        // Flush, to make sure messages without newlines get delivered
        self.pianobar_stdin.flush().await?;

        Ok(())
    }
}

pub struct PianobarStdoutHandler {
    pianobar_stdout: ChildStdout,
    pianobar_received_messages: broadcast::Sender<PianobarMessage>,
}

impl PianobarStdoutHandler {
    fn new(
        pianobar_stdout: ChildStdout,
        pianobar_received_messages: broadcast::Sender<PianobarMessage>,
    ) -> Self {
        Self {
            pianobar_stdout,
            pianobar_received_messages,
        }
    }
    async fn run(&mut self) -> Result<()> {
        parse_pianobar_messages(&mut self.pianobar_stdout, &self.pianobar_received_messages)
            .await?;
        Err(anyhow!("Pianobar process closed."))
    }
}

#[derive(Clone)]
pub struct PianobarController {
    // Wrapped in Mutex to prevent multiple people from sending simultaneously.
    pianobar_actor: Arc<Mutex<PianobarActor>>,
    pianobar_received_messages: broadcast::Sender<PianobarMessage>,
    pianobar_stdout_handler: Arc<Mutex<PianobarStdoutHandler>>,
}

impl PianobarController {
    pub fn start_pianobar_process(pianobar_command: &str) -> Result<(PianobarController, Child)> {
        // Start the pianobar process and get the handle to the stdin and stdout streams
        log::info!("Start pianobar process ...");
        let mut pianobar_process = Command::new(pianobar_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        let pianobar_stdin = pianobar_process
            .stdin
            .take()
            .ok_or(anyhow!("Unable to get pianobar stdin."))?;
        let pianobar_stdout = pianobar_process
            .stdout
            .take()
            .ok_or(anyhow!("Unable to get pianobar stdout."))?;

        // Create a broadcast channel for the communication with the stdout task
        let (pianobar_received_messages, _) = broadcast::channel(20);

        // Spawn the stdout handler task
        let pianobar_stdout_handler =
            PianobarStdoutHandler::new(pianobar_stdout, pianobar_received_messages.clone());

        // Create the pianobar actor
        let pianobar_actor = Arc::new(Mutex::new(PianobarActor::new(pianobar_stdin)));

        // Create the controller object
        Ok((
            PianobarController {
                pianobar_actor,
                pianobar_received_messages,
                pianobar_stdout_handler: Arc::new(Mutex::new(pianobar_stdout_handler)),
            },
            pianobar_process,
        ))
    }

    pub async fn take_actor(&self) -> tokio::sync::MutexGuard<'_, PianobarActor> {
        self.pianobar_actor.lock().await
    }

    pub fn subscribe(&self) -> broadcast::Receiver<PianobarMessage> {
        self.pianobar_received_messages.subscribe()
    }

    pub async fn run(&self) -> Result<()> {
        self.pianobar_stdout_handler.lock().await.run().await
    }
}
