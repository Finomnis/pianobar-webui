mod debug_printer;
mod manual_controller;
pub mod player_state;

use super::{PianobarController, PianobarMessage};
use anyhow::Result;

pub async fn plugins(pianobar: &PianobarController) -> Result<()> {
    tokio::try_join!(
        debug_printer::debug_printer(pianobar),
        manual_controller::manual_controller(pianobar)
    )?;

    Ok(())
}
