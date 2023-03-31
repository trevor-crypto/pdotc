use pdotc::client::{ApiBuilder, ClientError, Result, Signer, StdError};
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::ss58::Ss58Codec;
use pdotc::{Ed25519Public, Ed25519Signature, MultiAddress, UncheckedExtrinsic};
use serde_json::Value;
use sp_core::crypto::{AccountId32, Pair as _};
use sp_core::ed25519::Pair;
use sp_core::Decode;

struct PDotClient<HttpClient> {
    url: String,
    inner: HttpClient,
}

struct KeyStore {
    pub pair: Pair,
}

impl KeyStore {
    fn new(pair: Pair) -> Self {
        Self { pair }
    }
}

impl Signer for KeyStore {
    type SigBytes = [u8; 64];
    type PubBytes = [u8; 32];
    type Signature = Ed25519Signature;
    type Pub = Ed25519Public;

    fn _public(&self) -> std::result::Result<AccountId32, StdError> {
        let pub_key: Ed25519Public = self.pair.into();
        Ok(pub_key.into())
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

impl PDotClient<ureq::Agent> {
    fn polyx() -> Self {
        Self {
            inner: ureq::agent(),
            url: "http://testnet-rpc.polymesh.live/http".to_string(),
        }
    }
}

fn main() {
    let client = PDotClient::polyx();
    let seed = [
        185, 125, 217, 137, 121, 53, 179, 62, 100, 212, 10, 65, 202, 11, 43, 117, 21, 225, 24, 72,
        205, 210, 140, 24, 238, 50, 210, 70, 102, 185, 170, 42,
    ];
    let pair = Pair::from_seed(&seed);
    let keystore = KeyStore::new(pair);
    let primary_key = keystore.pair.public();
    let api = ApiBuilder::polymesh(&client)
        .signer(keystore)
        .build()
        .unwrap();
    let account: AccountId32 = primary_key.into();

    // get balance
    let balance = api.account_data(account, None).unwrap();
    dbg!(balance);

    // sign a tx
    let xt = api
        .balance_transfer(
            MultiAddress::Id(
                AccountId32::from_ss58check("5GHJEvkT9HngLP8kcaDyJ9o57xRzpvv5cvEoQkaBrZKQfBSx")
                    .unwrap(),
            ),
            10000,
            Some(0),
        )
        .expect("Created xt");
    let xt_hex = xt.as_hex();
    dbg!(&xt_hex);

    // get the fee for xt
    match api.fee_details(&xt_hex, None) {
        Ok(fees) => {
            dbg!(fees);
        }
        Err(e) => {
            dbg!(e);
        }
    }

    // decode xt
    assert_eq!(
        xt,
        UncheckedExtrinsic::decode(
            &mut hex::decode(xt_hex.trim_start_matches("0x"))
                .unwrap()
                .as_slice(),
        )
        .unwrap()
    );

    // send out the transfer xt
    // match client.send_extrinsic(&xt_hex) {
    // Ok(tx_hash) => {
    // dbg!(tx_hash);
    //}
    // Err(e) => {
    // dbg!(e);
    //}
}
