use pdotc::client::*;
use pdotc::rpc::{JsonRpcResponse, RpcClient};
use pdotc::{MultiAddress, MultiSignature};

struct PDotClient<HttpClient> {
    url: String,
    inner: HttpClient,
}

struct KeyStore;

impl Signer for KeyStore {
    fn sign(&self, _message: &[u8]) -> MultiSignature {
        todo!()
    }

    fn public(&self) -> MultiAddress {
        todo!()
    }
}

impl RpcClient for PDotClient<ureq::Agent> {
    fn post(&self, json_req: serde_json::Value) -> Result<JsonRpcResponse> {
        Ok(self
            .inner
            .post(&self.url)
            .send_json(json_req)
            .map_err(|e| ClientError::HttpClient(e.to_string()))?
            .into_json::<JsonRpcResponse>()?)
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

#[test]
fn test_client() {
    let client = PDotClient::default();
    let api = Api::<(), _>::new(client).unwrap();
    dbg!(api.genesis_hash);
    dbg!(api.runtime_version);
    let balance = api
        .account_data("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe")
        .unwrap();
    dbg!(balance);
}
