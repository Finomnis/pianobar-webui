use super::json_rpc::JsonRpcWebsocket;
use crate::PianobarActions;
use jsonrpc_core::{Error, ErrorCode, Params, Result};
use serde_json as json;
use std::sync::Arc;

macro_rules! bail {
    ($err:expr $(,)?) => {
        return Err($err);
    };
}

// Implement .to_json_error conversion function for internal errors
trait ResultToJsonError {
    fn to_json_error(&self) -> Result<()>;
}
impl ResultToJsonError for std::result::Result<(), anyhow::Error> {
    fn to_json_error(&self) -> Result<()> {
        match self {
            Ok(()) => Ok(()),
            Err(err) => Err(Error {
                code: ErrorCode::InternalError,
                message: err.to_string(),
                data: None,
            }),
        }
    }
}

pub fn register(handler: &mut JsonRpcWebsocket<Arc<PianobarActions>>) {
    handler.add_method("change_station", change_station);
}

struct ArgsExtractor {
    params: Params,
}

impl ArgsExtractor {
    pub fn new(params: Params, max_params: usize) -> Result<ArgsExtractor> {
        if match &params {
            Params::Array(arr) => (arr.len() > max_params),
            Params::Map(map) => (map.len() > max_params),
            Params::None => false,
        } {
            bail!(Error::invalid_params("Too many arguments"));
        }

        Ok(ArgsExtractor { params })
    }

    fn get<T>(&self, pos: usize, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = match &self.params {
            Params::Array(arr) => arr
                .get(pos)
                .ok_or(Error::invalid_params("Not enough arguments")),
            Params::Map(map) => map.get(name).ok_or(Error::invalid_params(format!(
                "Missing argument: '{}'",
                name
            ))),
            Params::None => Err(Error::new(ErrorCode::InvalidParams)),
        }?;
        json::value::from_value(value.clone())
            .or_else(|err| Err(Error::invalid_params(err.to_string())))
    }
}

async fn change_station(params: Params, actions: Arc<PianobarActions>) -> Result<json::Value> {
    let args = ArgsExtractor::new(params, 1)?;

    Ok(json::json!(actions
        .change_station(args.get(0, "station_id")?)
        .await
        .to_json_error()?))
}
