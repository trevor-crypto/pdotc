use std::fmt::Debug;

use parity_scale_codec::Decode;
pub use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Value};
use sp_core::storage::StorageKey;
use sp_core::H256;

use crate::client::{ClientError, Result};
use crate::utils::FromHexString;

pub trait RpcClient {
    fn post(&self, json_req: Value) -> Result<JsonRpcResponse>;

    fn send_extrinstic(&self, xt: &str) -> Result<String> {
        let json = author_submit_extrinsic(xt);
        Ok(self.post(json)?.into_string()?)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse {
    Success(JsonRpcSuccess),
    Error(JsonRpcError),
    String(String),
}

impl JsonRpcResponse {
    pub fn into_result<T: DeserializeOwned>(self) -> Result<T, ClientError> {
        match self {
            JsonRpcResponse::Success(s) => Ok(serde_json::from_value::<T>(s.result)?),
            JsonRpcResponse::Error(e) => Err(ClientError::JsonRpcError(e)),
            JsonRpcResponse::String(s) => Ok(serde_json::from_str(&s)?),
        }
    }

    pub fn into_string(self) -> Result<String, JsonRpcError> {
        match self {
            JsonRpcResponse::Success(s) => Ok(s.result.to_string()),
            JsonRpcResponse::Error(e) => Err(e),
            JsonRpcResponse::String(s) => Ok(s),
        }
    }

    pub fn decode_into<T: Decode>(self) -> Result<T, ClientError> {
        match self {
            JsonRpcResponse::Success(s) => {
                let v = Vec::from_hex(s.result.to_string())?;
                let t = Decode::decode(&mut v.as_slice())?;
                Ok(t)
            }
            JsonRpcResponse::Error(e) => Err(ClientError::JsonRpcError(e)),
            JsonRpcResponse::String(s) => Ok(Decode::decode(&mut s.as_bytes())?),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcSuccess {
    pub result: Value,
    pub jsonrpc: String,
    pub id: String,
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("Json RPC error: code {{error.code}}, message {{error.message}}, data: {{error.data:?}}")]
pub struct JsonRpcError {
    pub jsonrpc: String,
    pub id: String,
    pub error: RpcError,
}

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

// JSON RPC REQUESTS

pub fn chain_get_block_hash(number: Option<u32>) -> Value {
    chain_get_block_hash_with_id(number, 1)
}

pub fn chain_get_genesis_hash() -> Value {
    chain_get_block_hash(Some(0))
}

pub fn chain_get_block_hash_with_id(number: Option<u32>, id: u32) -> Value {
    json_req("chain_getBlockHash", vec![number], id)
}

pub fn state_get_runtime_version() -> Value {
    state_get_runtime_version_with_id(1)
}

pub fn state_get_runtime_version_with_id(id: u32) -> Value {
    json_req("state_getRuntimeVersion", vec![Value::Null], id)
}

pub fn state_get_storage(key: StorageKey, at_block: Option<H256>) -> Value {
    json_req(
        "state_getStorage",
        vec![to_value(key).unwrap(), to_value(at_block).unwrap()],
        1,
    )
}

pub fn payment_query_fee_details(xt_hex_prefixed: &str, at_block: Option<H256>) -> Value {
    json_req(
        "payment_queryFeeDetails",
        vec![
            to_value(xt_hex_prefixed).unwrap(),
            to_value(at_block).unwrap(),
        ],
        1,
    )
}

pub fn author_submit_extrinsic(xt_hex_prefixed: &str) -> Value {
    author_submit_extrinsic_with_id(xt_hex_prefixed, 3)
}

pub fn author_submit_extrinsic_with_id(xt_hex_prefixed: &str, id: u32) -> Value {
    json_req("author_submitExtrinsic", vec![xt_hex_prefixed], id)
}

fn json_req<S: Serialize>(method: &str, params: S, id: u32) -> Value {
    json!({
        "method": method,
        "params": params,
        "jsonrpc": "2.0",
        "id": id.to_string(),
    })
}
