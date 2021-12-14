use sp_core::crypto::AccountId32;
use sp_core::ecdsa::Public;
pub use sp_core::ecdsa::Signature;

use crate::pallets::storage::storage_key_account_balance;
use crate::rpc::{
    chain_get_genesis_hash, payment_query_fee_details, state_get_runtime_version,
    state_get_storage, JsonRpcError, RpcClient,
};
use crate::utils::FromHexString;
use crate::{AccountData, AccountInfo, FeeDetails, MultiSignature, RuntimeVersion, H256};

pub type Result<R, E = ClientError> = std::result::Result<R, E>;

type StdError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Network {
    Polkadot,
    Westend,
}

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
    #[error("parity SCALE codec decode error: {0}")]
    DecodeError(#[from] parity_scale_codec::Error),
    #[error("Signer not set")]
    NoSigner,
    #[error("Signer account does not exist on chain")]
    SignerAccountDoesNotExist,
    #[error(transparent)]
    Other(#[from] StdError),
}

/// A trait to implement on a keystore that can produce an ECDSA signature
pub trait Signer {
    /// Returns a 33 byte ECDSA public key
    fn _public(&self) -> std::result::Result<[u8; 33], StdError>;

    /// Returns a 65 byte compressed ECDSA signature
    fn _sign(&self, message: &[u8]) -> std::result::Result<[u8; 65], StdError>;

    fn public(&self) -> Result<Public> {
        Ok(Public(self._public()?))
    }

    fn sign(&self, message: &[u8]) -> Result<MultiSignature> {
        Ok(MultiSignature::Ecdsa(Signature(self._sign(message)?)))
    }
}

/// A struct to interface with a node's JsonRPC server
pub struct Api<'c, S, Client: RpcClient> {
    pub genesis_hash: H256,
    pub runtime_version: RuntimeVersion,
    pub signer: Option<S>,
    client: &'c Client,
    pub(crate) network: Network,
}

impl<'c, S, Client: RpcClient> Api<'c, S, Client> {
    pub fn polkadot(client: &'c Client) -> Result<Self> {
        Self::new(client, Network::Polkadot)
    }

    pub fn polkadot_with_signer(client: &'c Client, signer: S) -> Result<Self> {
        Self::new_with_signer(client, signer, Network::Polkadot)
    }

    pub fn westend(client: &'c Client) -> Result<Self> {
        Self::new(client, Network::Westend)
    }

    pub fn westend_with_signer(client: &'c Client, signer: S) -> Result<Self> {
        Self::new_with_signer(client, signer, Network::Westend)
    }

    fn new(client: &'c Client, network: Network) -> Result<Self> {
        let genesis_hash = Self::genesis_hash(client)?;
        let runtime_version = Self::runtime_version(client)?;
        Ok(Api {
            genesis_hash,
            runtime_version,
            signer: None,
            client,
            network,
        })
    }

    fn new_with_signer(client: &'c Client, signer: S, network: Network) -> Result<Self> {
        let mut client = Self::new(client, network)?;
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
    /// Returns None because the account can not exist
    pub fn account_data<A: Into<AccountId32>>(&self, address: A) -> Result<Option<AccountData>> {
        self.account_info(address).map(|o| o.map(|i| i.data))
    }

    /// Get account info for given address
    /// Returns None because the account can not exist
    pub fn account_info<A: Into<AccountId32>>(&self, address: A) -> Result<Option<AccountInfo>> {
        let storage_key = storage_key_account_balance(address.into().as_ref());

        let json = state_get_storage(storage_key, None);
        let info: Option<AccountInfo> = self.client.post(json)?.decode_into()?;

        Ok(info)
    }

    /// Calculate a fee for given extrinsic
    pub fn fee_details(&self, xt_hex_prefixed: &str, at_block: Option<H256>) -> Result<FeeDetails> {
        let jsonreq = payment_query_fee_details(xt_hex_prefixed, at_block);
        let fees = self.client.post(jsonreq)?.into_result()?;

        Ok(fees)
    }
}
