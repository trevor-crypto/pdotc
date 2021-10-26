use sp_core::crypto::{AccountId32, Ss58Codec};
pub use sp_core::ecdsa::Signature;
pub use sp_core::H256;

use crate::pallets::storage_key_account_balance;
use crate::rpc::{
    chain_get_genesis_hash, state_get_runtime_version, state_get_storage, JsonRpcError, RpcClient,
};
use crate::utils::FromHexString;
use crate::{AccountData, AccountInfo, MultiAddress, MultiSignature, RuntimeVersion};

pub type Result<R, E = ClientError> = std::result::Result<R, E>;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Inner http client error: {0}")]
    HttpClient(String),
    #[error("{0}")]
    JsonRpcError(#[from] JsonRpcError),
    #[error("Json parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Converting hex string to hash: {0}")]
    FromHex(#[from] hex::FromHexError),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Bad address: {0}")]
    BadAddressString(String),
    #[error("parity SCALE codec decode error: {0}")]
    DecodeError(#[from] parity_scale_codec::Error),
}

/// A trait to implement on a keystore that can produce an ECDSA signature
pub trait Signer {
    fn public(&self) -> MultiAddress;
    fn sign(&self, message: &[u8]) -> MultiSignature;
}

/// A struct to interface with a node's JsonRPC server
pub struct Api<S, Client: RpcClient> {
    pub genesis_hash: H256,
    pub runtime_version: RuntimeVersion,
    pub signer: Option<S>,
    client: Client,
}

impl<S, Client: RpcClient> Api<S, Client> {
    pub fn new(client: Client) -> Result<Self> {
        let genesis_hash = Self::genesis_hash(&client)?;
        let runtime_version = Self::runtime_version(&client)?;
        Ok(Api {
            genesis_hash,
            runtime_version,
            signer: None,
            client,
        })
    }

    pub fn new_with_signer(client: Client, signer: S) -> Result<Self> {
        let mut client = Self::new(client)?;
        client.signer = Some(signer);
        Ok(client)
    }

    fn genesis_hash(client: &Client) -> Result<H256> {
        let json = client.post(chain_get_genesis_hash())?.into_string()?;
        let hash = H256::from_hex(json)?;
        Ok(hash)
    }

    fn runtime_version(client: &Client) -> Result<RuntimeVersion> {
        client.post(state_get_runtime_version())?.into_result()
    }

    /// Get balances of given address
    pub fn account_data(&self, address: &str) -> Result<AccountData> {
        self.account_info(address).map(|i| i.data)
    }

    /// Get account info for given address
    pub fn account_info(&self, address: &str) -> Result<AccountInfo> {
        let account = AccountId32::from_string(address)
            .map_err(|_| ClientError::BadAddressString(address.to_string()))?;
        let storage_key = storage_key_account_balance(account.as_ref());

        let json = state_get_storage(storage_key, None);
        let info: AccountInfo = self.client.post(json)?.decode_into()?;

        dbg!(&info);

        Ok(info)
    }
}
