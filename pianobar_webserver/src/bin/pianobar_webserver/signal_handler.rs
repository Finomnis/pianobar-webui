use anyhow::{anyhow, bail, Result};
use tokio::signal::{self, unix::SignalKind};

async fn handle_signal(description: &str, kind: SignalKind) -> Result<()> {
    signal::unix::signal(kind)?
        .recv()
        .await
        .ok_or(anyhow!("Error while waiting for signals ..."))?;
    bail!("Received {}. Exiting program ...", description)
}

async fn handle_ctrl_c() -> Result<()> {
    signal::ctrl_c().await?;
    bail!("Ctrl-C pressed. Exiting program ...")
}

pub async fn handle_interrupt_signals() -> Result<()> {
    tokio::try_join!(
        handle_ctrl_c(),
        handle_signal("SIGHUP", SignalKind::hangup()),
        handle_signal("SIGINT", SignalKind::interrupt()),
        handle_signal("SIGPIPE", SignalKind::pipe()),
        handle_signal("SIGQUIT", SignalKind::quit()),
        handle_signal("SIGTERM", SignalKind::terminate()),
    )?;
    bail!("All signal handlers ended. Should never happen ...");
}
