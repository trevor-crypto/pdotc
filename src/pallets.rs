use parity_scale_codec::Encode;
use sp_core::crypto::AccountId32;

use crate::client::{Api, ClientError, Result, Signer};
use crate::network::SubstrateNetwork;
use crate::rpc::RpcClient;
use crate::{Era, GenericExtra, SignedPayload, UncheckedExtrinsic};

pub mod balances;
pub mod proxy;
pub mod staking;
pub mod storage;
pub mod timestamp;

pub(crate) type CallIndex = [u8; 2];

impl<S: Signer, Client: RpcClient, N: SubstrateNetwork> Api<'_, S, Client, N> {
    /// Creates and signs an extrinsic that can be submitted to a node
    pub fn create_xt<C: Encode + Clone>(&self, call: C) -> Result<UncheckedExtrinsic<C>> {
        self._create_xt(call, None)
    }

    /// Creates and signs an extrinsic that can be submitted to a node
    /// with a given nonce
    pub fn create_xt_with_nonce<C: Encode + Clone>(
        &self,
        call: C,
        nonce: u32,
    ) -> Result<UncheckedExtrinsic<C>> {
        self._create_xt(call, Some(nonce))
    }

    /// Creates and signs an extrinsic that can be submitted to a node
    pub(crate) fn _create_xt<C: Encode + Clone>(
        &self,
        call: C,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<C>> {
        let signature = if let Some(signer) = &self.signer {
            let gen_hash = self.genesis_hash;
            let runtime_version = self.runtime_version;
            let nonce = if let Some(nonce) = nonce {
                nonce
            } else {
                self.nonce()?
            };
            let extra = GenericExtra::new(Era::Immortal, nonce);
            let s_extra = (
                runtime_version.spec_version,
                runtime_version.transaction_version,
                gen_hash,
                gen_hash,
                (),
                (),
                (),
            );
            let raw_payload = SignedPayload::new(call.clone(), extra, s_extra);
            let from = signer.public()?.into();
            let sig = raw_payload.encoded(|payload| signer.sign(payload))?;
            Some((from, sig, extra))
        } else {
            None
        };

        Ok(UncheckedExtrinsic {
            signature,
            function: call,
        })
    }

    pub fn signer_account(&self) -> Result<AccountId32> {
        match &self.signer {
            Some(signer) => Ok(signer.public()?),
            None => Err(ClientError::NoSigner),
        }
    }

    pub fn nonce(&self) -> Result<u32> {
        let acct = self.signer_account()?;
        let info = self
            .account_info(acct, None)?
            .ok_or(ClientError::SignerAccountDoesNotExist)?;
        Ok(info.nonce)
    }
}
