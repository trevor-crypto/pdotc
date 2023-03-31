use pdotc::client::{ApiBuilder, ClientError, Result, Signer};
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::ss58::Ss58Codec;
use pdotc::{Ed25519Public, MultiAddress, UncheckedExtrinsic};
use serde_json::Value;
use sp_core::crypto::{AccountId32, Pair, UncheckedFrom};
use sp_core::Decode;

struct PDotClient<HttpClient> {
    url: String,
    inner: HttpClient,
}

struct KeyStore<P: Pair> {
    pub pair: P,
}

impl<P: Pair> KeyStore<P> {
    fn new(pair: P) -> Self {
        Self { pair }
    }
}

impl<P: Pair> Signer for KeyStore<P>
where
    [u8; 32]: From<<P as sp_core::Pair>::Public>,
    [u8; 64]: From<<P as sp_core::Pair>::Signature>,
{
    type SigBytes = [u8; 64];
    type PubBytes = [u8; 32];
    type Signature = sp_core::ed25519::Signature;
    type Pub = Ed25519Public;

    fn _public(
        &self,
    ) -> std::result::Result<AccountId32, Box<(dyn std::error::Error + Send + Sync + 'static)>>
    {
        let pub_bytes: Self::PubBytes = self.pair.public().into();
        let pub_key: sp_core::ed25519::Public = sp_core::ed25519::Public::unchecked_from(pub_bytes);
        Ok(pub_key.into())
    }

    fn _sign(
        &self,
        message: &[u8],
    ) -> std::result::Result<Self::SigBytes, Box<(dyn std::error::Error + Send + Sync + 'static)>>
    {
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
    let pair = sp_core::ed25519::Pair::from_seed(&seed);
    let keystore: KeyStore<sp_core::ed25519::Pair> = KeyStore::new(pair);
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
