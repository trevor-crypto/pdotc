use sp_core::storage::StorageKey;
use sp_core::{blake2_128, twox_128, twox_64};

pub(crate) fn storage_key_account_balance(account: &[u8]) -> StorageKey {
    let pallet = twox_128("System".as_bytes());
    let storage = twox_128("Account".as_bytes());
    let key_hash: Vec<u8> = blake2_128(account).iter().chain(account).cloned().collect();
    let key: Vec<u8> = pallet.into_iter().chain(storage).chain(key_hash).collect();
    StorageKey(key)
}

pub(crate) fn storage_proxy_proxies(account: &[u8]) -> StorageKey {
    let pallet = twox_128("Proxy".as_bytes());
    let storage = twox_128("Proxies".as_bytes());
    let key_hash: Vec<u8> = twox_64(account).iter().chain(account).cloned().collect();
    let key: Vec<u8> = pallet.into_iter().chain(storage).chain(key_hash).collect();
    StorageKey(key)
}

#[cfg(test)]
mod tests {
    use sp_core::crypto::{AccountId32, Ss58Codec};

    use super::*;

    fn check(f: fn(&[u8]) -> StorageKey, expected: &str) {
        let got = f(AccountId32::from_ss58check_with_version(
            "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe",
        )
        .unwrap()
        .0
        .as_ref())
        .0;
        assert_eq!(hex::encode(got), expected);
    }

    #[test]
    fn system_account_storage_key() {
        let expected = "26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9f4aa2c6a213b1188832a85b0b63fc95bff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f";
        check(storage_key_account_balance, expected);
    }

    #[test]
    fn proxy_proxies_storage_key() {
        let expected = "1809d78346727a0ef58c0fa03bafa3231d885dcfb277f185f2d8e62a5f290c855e63108ebcfb0e35ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f";
        check(storage_proxy_proxies, expected);
    }
}
