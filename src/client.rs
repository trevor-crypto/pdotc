use std::marker::PhantomData;

use sp_core::crypto::AccountId32;
use sp_core::ecdsa::Public;
pub use sp_core::ecdsa::Signature;

use crate::network::{Kusama, Polkadot, SubstrateNetwork, Westend};
use crate::pallets::storage::storage_key_account_balance;
use crate::rpc::{
    chain_get_block, chain_get_block_hash, chain_get_genesis_hash, payment_query_fee_details,
    state_get_runtime_version, state_get_storage, JsonRpcError, RpcClient,
};
use crate::utils::FromHexString;
use crate::{
    AccountData, AccountInfo, FeeDetails, MultiSignature, RuntimeVersion, SignedBlock, H256,
};

pub type Result<R, E = ClientError> = std::result::Result<R, E>;

type StdError = Box<dyn std::error::Error + Send + Sync>;

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

pub struct ApiBuilder;

impl<'c> ApiBuilder {
    pub fn polkadot<C: RpcClient>(client: &'c C) -> ApiBuilderWithClient<'c, C, Polkadot> {
        ApiBuilderWithClient {
            client,
            network: PhantomData,
        }
    }

    pub fn westend<C: RpcClient>(client: &'c C) -> ApiBuilderWithClient<'c, C, Westend> {
        ApiBuilderWithClient {
            client,
            network: PhantomData,
        }
    }

    pub fn kusama<C: RpcClient>(client: &'c C) -> ApiBuilderWithClient<'c, C, Kusama> {
        ApiBuilderWithClient {
            client,
            network: PhantomData,
        }
    }
}

pub struct ApiBuilderWithClient<'c, C: RpcClient, N: SubstrateNetwork> {
    client: &'c C,
    network: PhantomData<N>,
}

impl<'c, C: RpcClient, N: SubstrateNetwork> ApiBuilderWithClient<'c, C, N> {
    pub fn signer<S>(self, signer: S) -> ApiBuilderWithClientAndSigner<'c, S, C, N> {
        ApiBuilderWithClientAndSigner {
            client: self.client,
            network: PhantomData,
            signer: Some(signer),
        }
    }

    pub fn build<S>(self) -> Result<Api<'c, S, C, N>> {
        ApiBuilderWithClientAndSigner {
            client: self.client,
            network: PhantomData,
            signer: None,
        }
        .build()
    }
}

pub struct ApiBuilderWithClientAndSigner<'c, S, C: RpcClient, N: SubstrateNetwork> {
    client: &'c C,
    network: PhantomData<N>,
    signer: Option<S>,
}

impl<'c, S, C: RpcClient, N: SubstrateNetwork> ApiBuilderWithClientAndSigner<'c, S, C, N> {
    pub fn build(self) -> Result<Api<'c, S, C, N>> {
        let genesis_hash = genesis_hash(self.client)?;
        let runtime_version = runtime_version(self.client)?;
        Ok(Api {
            genesis_hash,
            runtime_version,
            signer: self.signer,
            client: self.client,
            network: PhantomData,
        })
    }
}

fn genesis_hash<C: RpcClient>(client: &C) -> Result<H256> {
    let json = client.post(chain_get_genesis_hash())?.into_result()?;
    let hash = H256::from_hex(json)?;
    Ok(hash)
}

fn runtime_version<C: RpcClient>(client: &C) -> Result<RuntimeVersion> {
    client.post(state_get_runtime_version())?.into_result()
}

/// A struct to interface with a node's JsonRPC server
pub struct Api<'c, S, C: RpcClient, Network: SubstrateNetwork> {
    pub(crate) genesis_hash: H256,
    pub(crate) runtime_version: RuntimeVersion,
    pub(crate) signer: Option<S>,
    pub(crate) client: &'c C,
    network: PhantomData<Network>,
}

impl<'c, S, C: RpcClient, N: SubstrateNetwork> Api<'c, S, C, N> {
    /// Get balances of given address
    /// Returns None because the account can not exist
    pub fn account_data<A: Into<AccountId32>>(
        &self,
        address: A,
        at_block: Option<H256>,
    ) -> Result<Option<AccountData>> {
        self.account_info(address, at_block)
            .map(|o| o.map(|i| i.data))
    }

    /// Get account info for given address
    /// Returns None because the account can not exist
    pub fn account_info<A: Into<AccountId32>>(
        &self,
        address: A,
        at_block: Option<H256>,
    ) -> Result<Option<AccountInfo>> {
        let storage_key = storage_key_account_balance(address.into().as_ref());

        let json = state_get_storage(storage_key, at_block);
        let info: Option<AccountInfo> = self.client.post(json)?.decode_into()?;

        Ok(info)
    }

    /// Gets block of specified hash or current block if `hash` is `None`
    pub fn block(&self, hash: Option<H256>) -> Result<SignedBlock> {
        self.client.post(chain_get_block(hash))?.into_result()
    }

    /// Gets block hash of block `number` or current block if `number` is `None`
    pub fn block_hash(&self, number: Option<u32>) -> Result<H256> {
        self.client
            .post(chain_get_block_hash(number))?
            .into_result()
    }

    /// Calculate a fee for given extrinsic
    pub fn fee_details(&self, xt_hex_prefixed: &str, at_block: Option<H256>) -> Result<FeeDetails> {
        let jsonreq = payment_query_fee_details(xt_hex_prefixed, at_block);
        let fees = self.client.post(jsonreq)?.into_result()?;

        Ok(fees)
    }
}
