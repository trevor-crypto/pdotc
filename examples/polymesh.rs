#![allow(unused)]
use parity_scale_codec::Decode;
use pdotc::client::{ApiBuilder, ClientError, Result, Signer};
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::ss58::Ss58Codec;
use pdotc::{
    blake2_256, public_into_account, Ed25519Sig, MultiAddress, Ss58AddressFormat,
    Ss58AddressFormatRegistry, UncheckedExtrinsic,
};
use secp256k1::{Message, Secp256k1, SecretKey};
use serde_json::Value;
use sp_core::crypto::AccountId32;
use sp_core::sr25519::{self, Pair};
use sp_core::Pair as TraitPair;

// Polymesh only supports ed25519 and sr25519
// Polymesh address: 5F6L9ofZJCLYuVYMrXZeywe6pxWUPNqetNwHtvo7jdsu4tYQ
const SEED_1: &str = "c7f1b0eee936031bdb0266090d0fa333c5ad33df5a83efc20ce9fb348317da7c";
const MNEMONIC: &str = "pony seed boat doll adapt lion dutch acquire furnace icon help bachelor";
// Polymesh address: 5Do6aEw8REe7b8aUX2oVZMXnujbcTtduiXQcmGa3Lnya5G8w
const SEED_2: &str = "0ad6778ab58908050d0dc6be266a70c759c176254e06e5c7052218449371108c";
// Polymesh address: 5FPDySfrBfGN3XA6thUgWmME4hZ8Q7TyZLMcQzirjNmjKC8y
const SEED_3: &str = "6d5321f010f3c2ab81508b74eb150f1a9252299690b27b10ef6045a9716ed869";
// Polymesh address: 5DNSQJirmaiKuRf9gxbQQWx7fWTPRorEDxYkicBcRngGz6Lj
const SEED_4: &str = "e02abef13d8b05e06c36c42d2b634d57a2c263dd35da81cc9b77cccd9b17b52c";
// Polymesh address: 5Ftp4PgyEafycLGWmUNasxJbanrF1kkK6FAvRdHo8J5vDSt8
const MNEMONIC_1: &str = "foam trim elegant fragile wise blade cause have chef ethics medal ramp";
const SEED_5: &str = "1d8820192af963f513a5f326d14af854c96779c0455324ac5052b9be81863442";
// Polymesh address: 5HSUdXTJ3xFEkFpiiGoHw36zrSrfTWHiu52qj8WqMotsrLRW
const SEED_6: &str = "d234cf221ddb00f6944d5d0f97836b2dcddb319e0e3bd88f51a321360215395e";

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
        let seed = hex::decode(SEED_6).unwrap();
        let key = SecretKey::from_slice(&seed).unwrap();
        Self::new(key)
    }
}

impl<'a> Signer<'a> for KeyStore {
    type Signature = Ed25519Sig;

    fn _public<const N: usize>(
        &self,
    ) -> std::result::Result<[u8; N], Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        todo!()
    }

    fn _sign(
        &self,
        message: &[u8],
    ) -> std::result::Result<[u8; 65], Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        todo!()
    }
}

// impl Signer for KeyStore {
//     fn _public(
//         &self,
//     ) -> std::result::Result<[u8; 33], Box<(dyn std::error::Error + Send +
// Sync + 'static)>> {         let secp = Secp256k1::new();
//         let pubkey = self.key.public_key(&secp);
//         Ok(pubkey.serialize())
//     }

//     fn _sign(
//         &self,
//         message: &[u8],
//     ) -> std::result::Result<[u8; 65], Box<(dyn std::error::Error + Send +
// Sync + 'static)>> {         let secp = Secp256k1::signing_only();
//         let digest = blake2_256(message);

//         let message = Message::from_slice(&digest)?;

//         let (rec_id, compact) = secp
//             .sign_ecdsa_recoverable(&message, &self.key)
//             .serialize_compact();
//         let mut sig = [0; 65];
//         sig[0..64].copy_from_slice(&compact);
//         sig[64] = rec_id.to_i32() as u8;
//         Ok(sig)
//     }
// }

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
    let client = PDotClient::polyx();
    let keystore = KeyStore::default();
    let api = ApiBuilder::polymesh(&client)
        .signer(keystore)
        .build()
        .unwrap();
    println!("Public: {:?}", api.signer);
    println!(
        "Signer_account: {:?}",
        api.signer_account()
            .unwrap()
            .to_ss58check_with_version(Ss58AddressFormat::custom(42))
    );
    println!(
        "Account Info: {:?}",
        api.account_info(
            AccountId32::from_ss58check_with_version(
                "5HSUdXTJ3xFEkFpiiGoHw36zrSrfTWHiu52qj8WqMotsrLRW"
            )
            .unwrap()
            .0,
            None
        )
    );

    // get balance
    let balance = api
        .account_data(
            AccountId32::from_ss58check("5HSUdXTJ3xFEkFpiiGoHw36zrSrfTWHiu52qj8WqMotsrLRW")
                .unwrap(),
            None,
        )
        .unwrap();
    dbg!(balance);

    // sign a tx
    let xt = api
        .balance_transfer(
            MultiAddress::Id(
                AccountId32::from_ss58check("5HSUdXTJ3xFEkFpiiGoHw36zrSrfTWHiu52qj8WqMotsrLRW")
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
