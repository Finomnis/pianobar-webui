use anyhow::{bail, Result};
use tokio::sync::watch;

pub struct CancelSignal {
    state_getter: watch::Receiver<Option<String>>,
    state_setter: watch::Sender<Option<String>>,
}

impl CancelSignal {
    pub fn new() -> CancelSignal {
        let (state_setter, state_getter) = watch::channel(None);
        CancelSignal {
            state_setter,
            state_getter,
        }
    }
    pub fn set(&self, msg: String) {
        if let Err(err) = self.state_setter.send(Some(msg)) {
            log::error!("Unable to send stop signal: {}", err);
        }
    }
    pub async fn wait(&self) -> Result<()> {
        let mut getter = self.state_getter.clone();
        loop {
            if let Some(msg) = getter.borrow().clone() {
                bail!("Stopped: {}", msg);
            }
            getter.changed().await?;
        }
    }
}
