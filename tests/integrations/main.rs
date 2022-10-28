#![feature(once_cell)]

use pdotc::blake2_256;
use pdotc::client::{ClientError, Result, Signer};
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use serde_json::{json, Value};

mod kusama;
mod polkadot;
mod westend;
mod xt;

const SEED_1: &str = "9d90b79e257eeb651e0f6759d14c35e5091161f97b079d6a7ca3645067c6ff3f";

pub struct PDotClient<HttpClient> {
    url: String,
    sidecar_url: String,
    inner: HttpClient,
}

pub struct KeyStore {
    key: SecretKey,
}

impl Default for KeyStore {
    fn default() -> Self {
        let seed = hex::decode(SEED_1).unwrap();
        let key = SecretKey::from_slice(&seed).unwrap();
        Self { key }
    }
}

impl Signer for KeyStore {
    fn _public(
        &self,
    ) -> std::result::Result<[u8; 33], Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        let secp = Secp256k1::new();
        let pubkey = PublicKey::from_secret_key(&secp, &self.key);
        Ok(pubkey.serialize())
    }

    fn _sign(
        &self,
        message: &[u8],
    ) -> std::result::Result<[u8; 65], Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        let secp = Secp256k1::signing_only();
        let digest = blake2_256(message);

        let message = Message::from_slice(&digest)?;

        let (rec_id, compact) = secp
            .sign_recoverable(&message, &self.key)
            .serialize_compact();
        let mut sig = [0; 65];
        sig[0..64].copy_from_slice(&compact);
        sig[64] = rec_id.to_i32() as u8;
        Ok(sig)
    }
}

impl RpcClient for PDotClient<ureq::Agent> {
    fn post(&self, json_req: serde_json::Value) -> Result<JsonRpcResponse> {
        let v: Value = self
            .inner
            .post(&self.url)
            .send_json(json_req)
            .map_err(|e| ClientError::HttpClient(e.to_string()))?
            .into_json()?;
        Ok(serde_json::from_value(v)?)
    }
}

impl RpcClient for &PDotClient<ureq::Agent> {
    fn post(&self, json_req: serde_json::Value) -> Result<JsonRpcResponse> {
        let v: Value = self
            .inner
            .post(&self.url)
            .send_json(json_req)
            .map_err(|e| ClientError::HttpClient(e.to_string()))?
            .into_json()?;
        Ok(serde_json::from_value(v)?)
    }
}

impl PDotClient<ureq::Agent> {
    pub fn dot() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://rpc.polkadot.io".to_string(),
            sidecar_url: "http://127.0.0.1:8080".to_string(),
        }
    }

    pub fn wnd() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://westend-rpc.polkadot.io".to_string(),
            sidecar_url: "http://127.0.0.1:8081".to_string(),
        }
    }

    pub fn ksm() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://kusama-rpc.polkadot.io".to_string(),
            sidecar_url: "http://127.0.0.1:8082".to_string(),
        }
    }

    pub fn sidecar_dry_run(&self, xt: String) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/transaction/dry-run", self.sidecar_url);
        let res = self
            .inner
            .post(&url)
            .send_json(json!({ "tx": xt }))?
            .into_json::<Value>()?;
        Ok(res)
    }
}

/// Validate by checking xt fee on blockchain
#[macro_export]
macro_rules! validate_xt {
    ($call:ident($($args:literal),*), $expected:literal) => {
        paste! {
            #[test]
            fn  [<test_ $call>]() {
                let xt = $crate::xt::$call(&API, $($args)*).call_as_hex();
                assert_eq!(xt, $expected);
            }

            #[test]
            fn  [<test_ $call _fee>]() {
                let xt = $crate::xt::$call(&API, $($args)*).as_hex();
                let res = API.fee_details(&xt, None);
                dbg!(&res);
                assert!(res.is_ok());
            }

            #[test]
            #[ignore = "requires sidecar running"]
            fn  [<test_ $call _dry_run>]() {
                let xt = $crate::xt::$call(&API, $($args)*).as_hex();
                let res = client().sidecar_dry_run(xt);
                dbg!(&res);
                assert!(res.is_ok());
            }
         }
    };
}
