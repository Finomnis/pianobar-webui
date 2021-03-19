use super::messages::{parse_pianobar_messages, PianobarMessage};
use pianobar_webserver::utils::cancel_signal::CancelSignal;

use anyhow::{anyhow, bail, Result};
use log;
use std::{process::Stdio, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, Command},
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

#[derive(Clone)]
pub struct PianobarController {
    // Wrapped in Mutex to prevent multiple people from sending simultaneously.
    pianobar_actor: Arc<Mutex<PianobarActor>>,
    pianobar_received_messages: broadcast::Sender<PianobarMessage>,
    pianobar_process_stopped: Arc<CancelSignal>,
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

        // Create a cancel signal that allowes the stdout handler task to stop the controller
        let pianobar_process_stopped = Arc::new(CancelSignal::new());

        // Spawn the stdout handler task
        {
            let process_stopped_setter = pianobar_process_stopped.clone();
            let pianobar_received_messages_clone = pianobar_received_messages.clone();
            tokio::spawn(async move {
                match parse_pianobar_messages(pianobar_stdout, pianobar_received_messages_clone)
                    .await
                {
                    Ok(()) => {
                        process_stopped_setter.set("Pianobar stdout task closed.".to_string())
                    }
                    Err(err) => process_stopped_setter.set(format!("{}", err)),
                };
            });
        }

        // Create the pianobar actor
        let pianobar_actor = Arc::new(Mutex::new(PianobarActor::new(pianobar_stdin)));

        // Create the controller object
        Ok((
            PianobarController {
                pianobar_actor,
                pianobar_received_messages,
                pianobar_process_stopped,
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

    pub async fn watch_pianobar_process_alive(&self) -> Result<()> {
        self.pianobar_process_stopped.wait().await
    }
}
