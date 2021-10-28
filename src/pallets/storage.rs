use serde::{Deserialize, Serialize};
use sp_core::{blake2_128, twox_128};

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct StorageKey(pub Vec<u8>);

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
