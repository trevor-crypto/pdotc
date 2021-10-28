use parity_scale_codec::Decode;
use pdotc::client::*;
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::{blake2_256, AccountId32, MultiAddress, Ss58Codec, UncheckedExtrinsic};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

// WND address: 5CsanGiE6kBWxdW7qWkxSN6ZnD5hrLCz5nj94qJrqknRn3Jq
const SEED_1: &str = "9d90b79e257eeb651e0f6759d14c35e5091161f97b079d6a7ca3645067c6ff3f";

struct PDotClient<HttpClient> {
    url: String,
    inner: HttpClient,
}

struct KeyStore {
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
    fn _public(&self) -> [u8; 33] {
        let secp = Secp256k1::new();
        let pubkey = PublicKey::from_secret_key(&secp, &self.key);
        pubkey.serialize()
    }

    fn _sign(&self, message: &[u8]) -> [u8; 65] {
        let secp = Secp256k1::signing_only();
        let digest = blake2_256(message);

        let message = Message::from_slice(&digest).expect("32 byte digest");

        let (rec_id, compact) = secp
            .sign_recoverable(&message, &self.key)
            .serialize_compact();
        let mut sig = [0; 65];
        sig[0..64].copy_from_slice(&compact);
        sig[64] = rec_id.to_i32() as u8;
        sig
    }
}

impl RpcClient for PDotClient<ureq::Agent> {
    fn post(&self, json_req: serde_json::Value) -> Result<JsonRpcResponse> {
        Ok(self
            .inner
            .post(&self.url)
            .send_json(json_req)
            .map_err(|e| ClientError::HttpClient(e.to_string()))?
            .into_json()?)
    }
}

impl Default for PDotClient<ureq::Agent> {
    fn default() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://westend-rpc.polkadot.io".to_string(),
        }
    }
}

fn main() {
    let client = PDotClient::default();
    let keystore = KeyStore::default();
    let api = Api::new_with_signer(&client, keystore).unwrap();

    // get balance
    let balance = api
        .account_data(
            AccountId32::from_string("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe").unwrap(),
        )
        .unwrap();
    dbg!(balance);

    // sign a tx
    let xt = api
        .balance_transfer(
            MultiAddress::Id(
                AccountId32::from_string("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe")
                    .unwrap(),
            ),
            1000,
        )
        .unwrap();
    let xt_hex = xt.as_hex();
    dbg!(&xt_hex);
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
    let tx_hash = client.send_extrinstic(&xt_hex).unwrap();
    dbg!(tx_hash);
}
