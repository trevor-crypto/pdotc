use parity_scale_codec::Encode;
use sp_core::storage::StorageKey;
use sp_core::{blake2_128, twox_128};

use crate::client::{Api, Signer};
use crate::rpc::RpcClient;
use crate::{Era, GenericExtra, SignedPayload, UncheckedExtrinsic};

pub mod balances;

pub(crate) const BALANCES_TRANSFER: [u8; 2] = [4, 0];

pub trait Composed: Encode {
    type ComposedType;
    fn compose(&self) -> Self::ComposedType;
}

impl<S: Signer, Client: RpcClient> Api<S, Client> {
    /// Creates and signs and extrinsic that can be submitted to a node
    pub fn create_xt<C: Composed>(&self, call: C) -> UncheckedExtrinsic<C::ComposedType> {
        let composed = call.compose();
        let gen_hash = self.genesis_hash;
        let runtime_version = self.runtime_version;
        let extra = GenericExtra::new(Era::Immortal, 0);
        let s_extra = (
            runtime_version.spec_version,
            runtime_version.transaction_version,
            gen_hash,
            gen_hash,
            (),
            (),
            (),
        );
        let raw_payload = SignedPayload::new(call, extra, s_extra);

        let signature = if let Some(signer) = &self.signer {
            let from = signer.public();
            let sig = raw_payload.encoded(|payload| signer.sign(payload));
            Some((from, sig, extra))
        } else {
            None
        };

        UncheckedExtrinsic {
            signature,
            function: composed,
        }
    }
}

pub(crate) fn storage_key_account_balance(account: &[u8]) -> StorageKey {
    storage_key("System", "Account", account)
}

fn storage_key(pallet: &str, storage: &str, account: &[u8]) -> StorageKey {
    let pallet = twox_128(pallet.as_bytes());
    let storage = twox_128(storage.as_bytes());
    let key_hash: Vec<u8> = blake2_128(account).iter().chain(account).cloned().collect();
    let key: Vec<u8> = pallet.into_iter().chain(storage).chain(key_hash).collect();
    StorageKey(key)
}
