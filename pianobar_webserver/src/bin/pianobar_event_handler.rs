use anyhow::{anyhow, Result};
use serde_json as json;
use std::io;
use std::io::prelude::*;
use std::net::Ipv4Addr;
use std::net::TcpStream;

use pianobar_webserver::default_config;
use pianobar_webserver::ui_state::{PianobarUiEvent, PianobarUiState};

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let command = std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .ok_or(anyhow!("Command line argument expected!"))?;

    let mut state = PianobarUiState::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let parts = line.splitn(2, '=').collect::<Vec<_>>();

        if let [key, value] = parts.as_slice() {
            state.insert(key.to_string(), json::Value::String(value.to_string()));
        };
    }

    // Merge station# entries to 'stations' list
    let mut stations = vec![];
    if let Some(json::Value::String(num_stations_str)) = state.remove("stationCount") {
        if let Ok(num_stations) = num_stations_str.parse::<usize>() {
            for station_id in 0..num_stations {
                let station_key = format!("station{}", station_id);
                let station_value = state
                    .remove(&station_key)
                    .ok_or(anyhow!("Station key does not exist!"))?;
                stations.push(station_value.clone());
            }
        }
    }
    state.insert("stations".to_string(), json::Value::Array(stations));

    let message = PianobarUiEvent { command, state };
    let json_message = json::to_vec(&message)?;

    let mut tcp_stream =
        TcpStream::connect((Ipv4Addr::new(127, 0, 0, 1), default_config::EVENT_PORT))?;

    tcp_stream.write(&json_message)?;

    Ok(())
}
