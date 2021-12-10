use parity_scale_codec::Encode;
use sp_core::crypto::AccountId32;

use crate::client::{Api, ClientError, Polkadot, Result, Signer, Westend};
use crate::rpc::RpcClient;
use crate::{public_into_account, Era, GenericExtra, SignedPayload, UncheckedExtrinsic};

pub mod balances;
pub mod staking;
pub mod storage;

pub(crate) type CallIndex = [u8; 2];

pub trait NetworkPallets {
    const BALANCE_PALLET_IDX: u8;
    const STAKING_PALLET_IDX: u8;
}

impl NetworkPallets for Polkadot {
    const BALANCE_PALLET_IDX: u8 = 5;
    const STAKING_PALLET_IDX: u8 = 7;
}

impl NetworkPallets for Westend {
    const BALANCE_PALLET_IDX: u8 = 4;
    const STAKING_PALLET_IDX: u8 = 6;
}

impl<S: Signer, Client: RpcClient, Network: NetworkPallets> Api<'_, S, Client, Network> {
    /// Creates and signs an extrinsic that can be submitted to a node
    pub fn create_xt<C: Encode + Clone>(&self, call: C) -> Result<UncheckedExtrinsic<C>> {
        let gen_hash = self.genesis_hash;
        let runtime_version = self.runtime_version;
        let extra = GenericExtra::new(Era::Immortal, self.nonce().unwrap_or_default());
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

        let signature = if let Some(signer) = &self.signer {
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
            Some(signer) => Ok(public_into_account(signer.public()?)),
            None => Err(ClientError::NoSigner),
        }
    }

    pub fn nonce(&self) -> Result<u32> {
        let acct = self.signer_account()?;
        let info = self
            .account_info(acct)?
            .ok_or(ClientError::SignerAccountDoesNotExist)?;
        Ok(info.nonce)
    }
}
