use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PianobarUiEvent {
    pub command: String,
    pub state: PianobarUiState,
}

impl From<PianobarUiEvent> for json::Map<String, json::Value> {
    fn from(event: PianobarUiEvent) -> Self {
        let mut result = json::Map::new();
        result.insert("command".to_string(), json::Value::String(event.command));
        result.insert("state".to_string(), json::Value::Object(event.state));
        result
    }
}

pub type PianobarUiState = json::Map<String, json::Value>;
