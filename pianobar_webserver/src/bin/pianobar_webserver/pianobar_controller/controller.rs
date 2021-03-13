use super::manual_controller::manual_controller;
use super::pianobar_stdout::{process_pianobar_output, PianobarMessage};
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

pub struct PianobarController {
    _pianobar_process: Child,
    // Wrapped in Mutex to prevent multiple people from sending simultaneously.
    pianobar_actor: Arc<Mutex<PianobarActor>>,
    pianobar_received_messages: broadcast::Sender<PianobarMessage>,
    cancel_signal: Arc<CancelSignal>,
}

impl PianobarController {
    pub fn new(pianobar_command: &str) -> Result<PianobarController> {
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
        let cancel_signal = Arc::new(CancelSignal::new());

        // Spawn the stdout handler task
        let cancel_signal_setter = cancel_signal.clone();
        let pianobar_received_messages_clone = pianobar_received_messages.clone();
        tokio::spawn(async move {
            match process_pianobar_output(pianobar_stdout, pianobar_received_messages_clone).await {
                Ok(()) => cancel_signal_setter.set("Pianobar stdout task closed.".to_string()),
                Err(err) => cancel_signal_setter.set(format!("{}", err)),
            };
        });

        // Create the pianobar actor
        let pianobar_actor = Arc::new(Mutex::new(PianobarActor::new(pianobar_stdin)));

        // Create the controller object
        Ok(PianobarController {
            _pianobar_process: pianobar_process,
            pianobar_actor,
            pianobar_received_messages,
            cancel_signal,
        })
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

    pub async fn take_actor(&self) -> tokio::sync::MutexGuard<'_, PianobarActor> {
        self.pianobar_actor.lock().await
    }

    pub async fn run(&self) -> Result<()> {
        tokio::try_join!(
            self.control_logic(),
            manual_controller(self),
            self.cancel_signal.wait(),
        )?;

        bail!("All controller tasks ended unexpectedly.");
    }
}
