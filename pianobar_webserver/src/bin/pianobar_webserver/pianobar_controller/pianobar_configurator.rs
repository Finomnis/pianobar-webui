use anyhow::{anyhow, bail, Result};
use ini::Ini;

use log;
use std::path::Path;

fn set_event_command(config: &mut Ini) -> Result<()> {
    // set event_command path
    let event_handler_path = std::env::current_exe()?
        .canonicalize()?
        .parent()
        .ok_or(anyhow!("Unable to get pianobar_event_handler directory!"))?
        .join("pianobar_event_handler");

    let event_handler_path_string = event_handler_path
        .to_str()
        .ok_or(anyhow!("Unable to stringify path!"))?;

    if !event_handler_path.exists() {
        bail!("'{}' does not exist!", event_handler_path_string);
    }

    config
        .with_general_section()
        .set("event_command", event_handler_path_string);

    Ok(())
}

pub fn set_pianobar_configs(config_file: &str) -> Result<()> {
    let config_path_expanded = shellexpand::tilde(config_file).to_string();
    let config_path = Path::new(&config_path_expanded);

    if !config_path.exists() {
        bail!(
            "Pianobar config ({}) does not exist! Please create it.",
            config_file
        );
    }

    let mut config = Ini::load_from_file(config_path)?;

    if let Err(err) = set_event_command(&mut config) {
        log::warn!(
            "------------------------------------------------------------------------------"
        );
        log::warn!("Unable to set the event_command config: {}", err);
        log::warn!("For this program to work, the event_command config needs to be set manually.");
        log::warn!(
            "Please set it to the absolute path of the \"pianobar_event_handler\" executable."
        );
        log::warn!(
            "------------------------------------------------------------------------------"
        );
    }

    let mut output = Vec::new();
    config.write_to(&mut output)?;
    match String::from_utf8(output) {
        Ok(output_str) => {
            log::debug!("Wrote config file:\n{}", output_str);
        }
        Err(err) => {
            log::debug!(
                "Wrote config file! Doesn't seem to be utf8: {}\n{:?}",
                err.utf8_error(),
                err.as_bytes()
            );
        }
    };

    config.write_to_file(config_path)?;

    Ok(())
}
