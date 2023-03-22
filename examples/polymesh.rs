#![allow(unused)]
use parity_scale_codec::Decode;
use pdotc::client::{ApiBuilder, ClientError, Result, Signer};
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::ss58::Ss58Codec;
use pdotc::{
    blake2_256, public_into_account, MultiAddress, Ss58AddressFormat, Ss58AddressFormatRegistry,
    UncheckedExtrinsic,
};
use secp256k1::{Message, Secp256k1, SecretKey};
use serde_json::Value;
use sp_core::crypto::AccountId32;
use sp_core::sr25519::{self, Pair};
use sp_core::Pair as TraitPair;

// Polymesh address: 5F6L9ofZJCLYuVYMrXZeywe6pxWUPNqetNwHtvo7jdsu4tYQ
const SEED_1: &str = "c7f1b0eee936031bdb0266090d0fa333c5ad33df5a83efc20ce9fb348317da7c";
const MNEMONIC: &str = "pony seed boat doll adapt lion dutch acquire furnace icon help bachelor";
// Polymesh address: 5Do6aEw8REe7b8aUX2oVZMXnujbcTtduiXQcmGa3Lnya5G8w
const SEED_2: &str = "0ad6778ab58908050d0dc6be266a70c759c176254e06e5c7052218449371108c";
// Polymesh address: 5FPDySfrBfGN3XA6thUgWmME4hZ8Q7TyZLMcQzirjNmjKC8y
const SEED_3: &str = "6d5321f010f3c2ab81508b74eb150f1a9252299690b27b10ef6045a9716ed869";
// Polymesh address: 5DNSQJirmaiKuRf9gxbQQWx7fWTPRorEDxYkicBcRngGz6Lj
const SEED_4: &str = "e02abef13d8b05e06c36c42d2b634d57a2c263dd35da81cc9b77cccd9b17b52c";

struct PDotClient<HttpClient> {
    url: String,
    inner: HttpClient,
}

#[derive(Clone, Debug)]
struct KeyStore {
    pub key: SecretKey,
}

impl KeyStore {
    fn new(key: SecretKey) -> Self {
        Self { key }
    }
}

impl Default for KeyStore {
    fn default() -> Self {
        let seed = hex::decode(SEED_4).unwrap();
        let key = SecretKey::from_slice(&seed).unwrap();
        Self::new(key)
    }
}

impl Signer for KeyStore {
    fn _public(
        &self,
    ) -> std::result::Result<[u8; 33], Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        let secp = Secp256k1::new();
        let pubkey = self.key.public_key(&secp);
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
            .sign_ecdsa_recoverable(&message, &self.key)
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

impl PDotClient<ureq::Agent> {
    fn polyx() -> Self {
        Self {
            inner: ureq::agent(),
            // Testnet API endpoint: https://testnet-rpc.polymesh.live/http
            url: "https://testnet-rpc.polymesh.live/http".to_string(),
        }
    }
}

fn main() {
    // let seed = [
    // 10, 214, 119, 138, 181, 137, 8, 5, 13, 13, 198, 190, 38, 106, 112, 199, 89,
    // 193, 118, 37, 78, 6, 229, 199, 5, 34, 24, 68, 147, 113, 16, 140,
    //];
    // let key = SecretKey::from_slice(&seed).unwrap();
    // let pub_key = key.public_key(&Secp256k1::new());
    let client = PDotClient::polyx();
    let keystore = KeyStore::default();
    // println!(
    //     "Public: {:?}",
    //     hex::encode(blake2_256(&keystore.public().unwrap().0))
    // );
    // let public: sr25519::Public =
    //     sr25519::Public::from_raw(blake2_256(&keystore.public().unwrap().0));
    // let pair = Pair::from_seed_slice(&hex::decode(SEED).unwrap()).unwrap();
    // println!("Public Pair {:?}", pair.public());
    // let acct: AccountId32 = public.into();
    // println!("Account: {acct:?}");
    println!("PUB_KEY: {:?}", keystore.public());
    let api = ApiBuilder::polymesh(&client)
        .signer(keystore)
        .build()
        .unwrap();
    let version = AccountId32::from_ss58check_with_version(
        "5F6L9ofZJCLYuVYMrXZeywe6pxWUPNqetNwHtvo7jdsu4tYQ",
    )
    .unwrap()
    .1;
    println!("Version: {:?}", version);
    println!("Public: {:?}", api.signer);
    println!(
        "Signer_account: {:?}",
        api.signer_account()
            .unwrap()
            .to_ss58check_with_version(Ss58AddressFormat::custom(42))
    );
    let res = AccountId32::from_ss58check_with_version(
        "5F6L9ofZJCLYuVYMrXZeywe6pxWUPNqetNwHtvo7jdsu4tYQ",
    )
    .unwrap()
    .0;
    let bytes: &[u8] = res.as_ref();
    println!("AccountId32: {:?}", bytes);
    println!(
        "Account Info: {:?}",
        api.account_info(
            AccountId32::from_ss58check_with_version(
                "5F6L9ofZJCLYuVYMrXZeywe6pxWUPNqetNwHtvo7jdsu4tYQ"
            )
            .unwrap()
            .0,
            None
        )
    );

    // get balance
    let balance = api
        .account_data(
            AccountId32::from_ss58check("5F6L9ofZJCLYuVYMrXZeywe6pxWUPNqetNwHtvo7jdsu4tYQ")
                .unwrap(),
            None,
        )
        .unwrap();
    dbg!(balance);

    // sign a tx
    let xt = api
        .balance_transfer(
            MultiAddress::Id(
                AccountId32::from_ss58check("5F6L9ofZJCLYuVYMrXZeywe6pxWUPNqetNwHtvo7jdsu4tYQ")
                    .unwrap(),
            ),
            10000,
            None,
        )
        .expect("Created xt");
    let xt_hex = xt.as_hex();
    dbg!(&xt_hex);

    // get the fee for xt
    // match api.fee_details(&xt_hex, None) {
    //     Ok(fees) => {
    //         dbg!(fees);
    //     }
    //     Err(e) => {
    //         dbg!(e);
    //     }
    // }

    // decode xt
    // assert_eq!(
    // xt,
    // UncheckedExtrinsic::decode(
    //&mut hex::decode(xt_hex.trim_start_matches("0x"))
    //.unwrap()
    //.as_slice(),
    //)
    //.unwrap()
    //);

    // send out the transfer xt
    match client.send_extrinsic(&xt_hex) {
        Ok(tx_hash) => {
            dbg!(tx_hash);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
