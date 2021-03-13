mod debug_printer;
mod manual_controller;

use super::PianobarController;
use anyhow::Result;

pub async fn plugins(pianobar: &PianobarController) -> Result<()> {
    tokio::try_join!(
        debug_printer::debug_printer(pianobar),
        manual_controller::manual_controller(pianobar)
    )?;

    Ok(())
}
