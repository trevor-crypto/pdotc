use parity_scale_codec::Decode;
use pdotc::client::*;
use pdotc::pallets::staking::RewardDestination;
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::{blake2_256, MultiAddress, UncheckedExtrinsic};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use serde_json::Value;
use sp_core::crypto::{AccountId32, Ss58Codec};

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
        dbg!(&v);
        Ok(serde_json::from_value(v)?)
    }
}

impl PDotClient<ureq::Agent> {
    fn dot() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://rpc.polkadot.io".to_string(),
        }
    }

    fn wnd() -> Self {
        Self {
            inner: ureq::agent(),
            url: "https://westend-rpc.polkadot.io".to_string(),
        }
    }
}

fn main() {
    let client = PDotClient::wnd();
    let keystore = KeyStore::default();
    let api = ApiBuilder::westend(&client)
        .signer(keystore)
        .build()
        .unwrap();

    // get balance
    let balance = api
        .account_data(
            AccountId32::from_ss58check_with_version(
                "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe",
            )
            .unwrap()
            .0,
            None,
        )
        .unwrap();
    dbg!(balance);

    // sign a tx
    let xt = api
        .balance_transfer(
            MultiAddress::Id(
                AccountId32::from_ss58check_with_version(
                    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe",
                )
                .unwrap()
                .0,
            ),
            1000,
        )
        .expect("Created xt");
    let xt_hex = xt.as_hex();
    dbg!(&xt_hex);

    // get the fee for xt
    let fees = api.fee_details(&xt_hex, None).unwrap();
    dbg!(fees);

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

    // staking bond
    let bond_xt_hex = api
        .staking_bond(
            MultiAddress::Id(
                AccountId32::from_ss58check_with_version(
                    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe",
                )
                .unwrap()
                .0,
            ),
            1000,
            RewardDestination::Stash,
        )
        .expect("Created xt")
        .as_hex();
    dbg!(&bond_xt_hex);

    // staking unbond
    let unbond_xt_hex = api.staking_unbond(1000).expect("Created xt").as_hex();
    dbg!(&unbond_xt_hex);

    // send out the transfer xt
    // let tx_hash = client.send_extrinstic(&xt_hex).unwrap();
    // dbg!(tx_hash);

    println!("Polkadot client");

    let client = PDotClient::dot();
    let keystore = KeyStore::default();
    let api = ApiBuilder::polkadot(&client)
        .signer(keystore)
        .build()
        .unwrap();

    // sign a tx
    let xt = api
        .balance_transfer(
            MultiAddress::Id(
                AccountId32::from_ss58check_with_version(
                    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe",
                )
                .unwrap()
                .0,
            ),
            1000,
        )
        .expect("Created xt");
    let xt_hex = xt.as_hex();
    dbg!(&xt_hex);

    // get the fee for xt
    let fees = api.fee_details(&xt_hex, None).unwrap();
    dbg!(fees);

    let bond_xt_hex = api
        .staking_bond(
            MultiAddress::Id(
                AccountId32::from_ss58check_with_version(
                    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe",
                )
                .unwrap()
                .0,
            ),
            1000,
            RewardDestination::Stash,
        )
        .expect("Created xt")
        .as_hex();
    dbg!(&bond_xt_hex);

    let res = api.block(None).expect("current block");
    dbg!(res.block.timestamp(), res.block.header);

    let proxies = api
        .proxies(
            AccountId32::from_ss58check_with_version(
                "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe",
            )
            .unwrap()
            .0,
        )
        .expect("Proxy account list");
    dbg!(proxies);
}
