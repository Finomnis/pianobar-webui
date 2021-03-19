pub mod actions;
mod debug_printer;
mod manual_controller;
pub mod player_state;

use super::{PianobarActor, PianobarController, PianobarMessage};
use anyhow::Result;

pub async fn plugins(pianobar: &PianobarController) -> Result<()> {
    // TODO remove this and replace it with a direct instantiation in main()
    // make the mods public
    tokio::try_join!(
        debug_printer::debug_printer(pianobar),
        manual_controller::manual_controller(pianobar)
    )?;

    Ok(())
}
