#![feature(lazy_cell)]

use pdotc::client::{ClientError, Result, Signer, StdError};
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::{public_into_account, AccountId32, EcdsaPublic, EcdsaSignature};
use serde_json::Value;
use sp_core::crypto::Pair as _;
use sp_core::ecdsa::Pair;

mod kusama;
mod polkadot;
mod westend;
mod xt;

const SEED_1: &str = "9d90b79e257eeb651e0f6759d14c35e5091161f97b079d6a7ca3645067c6ff3f";

pub struct PDotClient<HttpClient> {
    url: String,
    inner: HttpClient,
}

struct KeyStore {
    pair: Pair,
}

impl Default for KeyStore {
    fn default() -> Self {
        let seed = hex::decode(SEED_1).unwrap();
        let pair = Pair::from_seed_slice(&seed).unwrap();
        Self { pair }
    }
}

impl Signer for KeyStore {
    type SigBytes = [u8; 65];
    type PubBytes = [u8; 33];
    type Signature = EcdsaSignature;
    type Pub = EcdsaPublic;

    fn _public(&self) -> std::result::Result<AccountId32, StdError> {
        let pub_key: EcdsaPublic = self.pair.clone().into();
        Ok(public_into_account(pub_key))
    }

    fn _sign(&self, message: &[u8]) -> std::result::Result<Self::SigBytes, StdError> {
        Ok(self.pair.sign(message).into())
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
        }
    }

    pub fn wnd() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://westend-rpc.polkadot.io".to_string(),
        }
    }

    pub fn ksm() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://kusama-rpc.polkadot.io".to_string(),
        }
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
         }
    };
}

#[macro_export]
macro_rules! get_balance {
    ($addr:literal) => {
        #[test]
        fn get_balance() {
            let info = API
                .account_data(
                    AccountId32::from_ss58check_with_version($addr).unwrap().0,
                    None,
                )
                .unwrap();
            dbg!(info);
        }
    };
}
