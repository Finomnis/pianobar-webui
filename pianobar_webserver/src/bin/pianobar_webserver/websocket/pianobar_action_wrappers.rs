use super::json_rpc::JsonRpcWebsocket;
use crate::PianobarActions;
use jsonrpc_core as jsonrpc;
use serde_json as json;
use std::sync::Arc;

pub fn register_actions(handler: &mut JsonRpcWebsocket<Arc<PianobarActions>>) {
    handler.add_method("meaning_of_life", meaning_of_life);
}

async fn meaning_of_life(
    a: jsonrpc::Params,
    actions: Arc<PianobarActions>,
) -> jsonrpc::Result<json::Value> {
    println!("{:?}", a);
    Ok(json::json!(42))
}
