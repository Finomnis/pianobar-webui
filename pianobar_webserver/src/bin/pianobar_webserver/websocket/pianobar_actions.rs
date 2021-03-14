use super::json_rpc::JsonRpcWebsocket;
use crate::PianobarActions;
use jsonrpc_core as jsonrpc;
use jsonrpc_core::{Error, ErrorCode, Result};
use serde_json as json;
use std::sync::Arc;

macro_rules! bail {
    ($err:expr $(,)?) => {
        return jsonrpc::Result::Err($err);
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
            Err(err) => jsonrpc::Result::Err(Error {
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

fn get_arg<T>(params: &jsonrpc::Params, pos: usize, name: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let value = match params {
        jsonrpc::Params::Array(arr) => arr.get(pos).ok_or(Error {
            code: ErrorCode::InvalidParams,
            message: format!(
                "{}: Not enough arguments",
                ErrorCode::InvalidParams.description(),
            ),
            data: None,
        }),
        jsonrpc::Params::Map(map) => map.get(name).ok_or(Error {
            code: ErrorCode::InvalidParams,
            message: format!(
                "{}: Missing argument: '{}'",
                ErrorCode::InvalidParams.description(),
                name
            ),
            data: None,
        }),
        jsonrpc::Params::None => Err(Error::new(ErrorCode::InvalidParams)),
    }?;
    match json::value::from_value(value.clone()) {
        Ok(val) => Ok(val),
        Err(err) => Err(Error {
            code: ErrorCode::InvalidParams,
            message: err.to_string(),
            data: None,
        }),
    }
}

async fn change_station(
    params: jsonrpc::Params,
    actions: Arc<PianobarActions>,
) -> Result<json::Value> {
    // let a = get_arg()

    // let station_id = 0;

    Ok(json::json!(actions
        .change_station(get_arg(&params, 0, "station_id")?)
        .await
        .to_json_error()?))
}
