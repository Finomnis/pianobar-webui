use super::super::plugins;
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

pub struct PianobarController {
    pianobar_process: Arc<Mutex<Child>>,
    // Wrapped in Mutex to prevent multiple people from sending simultaneously.
    pianobar_actor: Arc<Mutex<PianobarActor>>,
    pianobar_received_messages: broadcast::Sender<PianobarMessage>,
    cancel_signal: Arc<CancelSignal>,
    pianobar_stdout_task: tokio::task::JoinHandle<()>,
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
        let pianobar_stdout_task = tokio::spawn(async move {
            match parse_pianobar_messages(pianobar_stdout, pianobar_received_messages_clone).await {
                Ok(()) => cancel_signal_setter.set("Pianobar stdout task closed.".to_string()),
                Err(err) => cancel_signal_setter.set(format!("{}", err)),
            };
        });

        // Create the pianobar actor
        let pianobar_actor = Arc::new(Mutex::new(PianobarActor::new(pianobar_stdin)));

        // Create the controller object
        Ok(PianobarController {
            pianobar_process: Arc::new(Mutex::new(pianobar_process)),
            pianobar_actor,
            pianobar_received_messages,
            cancel_signal,
            pianobar_stdout_task,
        })
    }

    pub async fn take_actor(&self) -> tokio::sync::MutexGuard<'_, PianobarActor> {
        self.pianobar_actor.lock().await
    }

    pub fn subscribe(&self) -> broadcast::Receiver<PianobarMessage> {
        self.pianobar_received_messages.subscribe()
    }

    pub async fn run(&self) -> Result<()> {
        tokio::try_join!(plugins::plugins(self), self.cancel_signal.wait())?;
        bail!("All controller tasks ended unexpectedly.");
    }

    async fn kill(&self) -> Result<()> {
        self.pianobar_process.lock().await.kill().await?;
        Ok(())
    }
}

impl Drop for PianobarController {
    fn drop(&mut self) {
        // Stop pianobar stdout parser task
        self.pianobar_stdout_task.abort();

        // Stop pianobar process
        match tokio::task::block_in_place(|| futures::executor::block_on(self.kill())) {
            Ok(()) => log::info!("Killed pianobar process."),
            Err(err) => log::warn!("Unable to kill pianobar: {}", err),
        }
    }
}
