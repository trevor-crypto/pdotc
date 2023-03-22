use std::fmt::Debug;
use std::str::FromStr;

use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::AccountId32;

use crate::client::{Api, Result, Signer};
use crate::network::SubstrateNetwork;
use crate::pallets::storage::storage_proxy_proxies;
use crate::pallets::CallIndex;
use crate::rpc::{state_get_storage, RpcClient};
use crate::UncheckedExtrinsic;

pub type ComposedProxyRemoveProxies = CallIndex;

/// Proxy types.
/// Governance is not available on Westend.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode)]
pub enum ProxyType {
    Any,
    NonTransfer,
    Governance,
    Staking,
}

impl FromStr for ProxyType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ptype =
            match s {
                "Any" => ProxyType::Any,
                "NonTransfer" => ProxyType::NonTransfer,
                "Governance" => ProxyType::Governance,
                "Staking" => ProxyType::Staking,
                _ => return Err(
                    "Invalid ProxyType. Expecting 'Any', 'NonTransfer', 'Governance' or 'Staking'",
                ),
            };
        Ok(ptype)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode)]
pub enum WestendProxyType {
    Any,
    NonTransfer,
    Staking,
}

impl FromStr for WestendProxyType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ptype = match s {
            "Any" => WestendProxyType::Any,
            "NonTransfer" => WestendProxyType::NonTransfer,
            "Staking" => WestendProxyType::Staking,
            _ => {
                return Err(
                    "Invalid WestendProxyType. Expecting 'Any', 'NonTransfer', or 'Staking'",
                )
            }
        };
        Ok(ptype)
    }
}

/// The parameters under which a particular account has a proxy relationship
/// with some other account.
/// https://github.com/paritytech/substrate/blob/master/frame/proxy/src/lib.rs
#[derive(Debug, Encode, Decode, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ProxyDefinition<AccountId, ProxyType, BlockNumber> {
    /// The account which may act on behalf of another.
    pub delegate: AccountId,
    /// A value defining the subset of calls that it is allowed to make.
    pub proxy_type: ProxyType,
    /// The number of blocks that an announcement must be in place for before
    /// the corresponding call may be dispatched. If zero, then no
    /// announcement is needed.
    pub delay: BlockNumber,
}

#[allow(clippy::type_complexity)]
impl<S: for<'a> Signer<'a>, C: RpcClient, N: SubstrateNetwork> Api<'_, S, C, N> {
    /// Register a proxy account for the sender that is able to make calls on
    /// its behalf.
    ///
    /// You may use `FromStr::from_str("...")` for `delegate` and `proxy_type`
    /// to be agnostic of Polkadot/Kusama and Westend types
    pub fn add_proxy(
        &self,
        delegate: N::ProxyDelegateType,
        proxy_type: N::ProxyTypeType,
        delay: u32,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<(CallIndex, N::ProxyDelegateType, N::ProxyTypeType, u32)>> {
        let call = (
            [N::PROXY_PALLET_IDX, N::PROXY_ADD_PROXY],
            delegate,
            proxy_type,
            delay,
        );
        self._create_xt(call, nonce)
    }

    /// Register a proxy account for the sender that is able to make calls on
    /// its behalf.
    ///
    /// You may use `FromStr::from_str("...")` for `delegate` and `proxy_type`
    /// to be agnostic of Polkadot/Kusama and Westend types
    pub fn remove_proxy(
        &self,
        delegate: N::ProxyDelegateType,
        proxy_type: N::ProxyTypeType,
        delay: u32,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<(CallIndex, N::ProxyDelegateType, N::ProxyTypeType, u32)>> {
        let call = (
            [N::PROXY_PALLET_IDX, N::PROXY_REMOVE_PROXY],
            delegate,
            proxy_type,
            delay,
        );
        self._create_xt(call, nonce)
    }

    /// Unregister all proxy accounts for the sender.
    pub fn remove_proxies(
        &self,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedProxyRemoveProxies>> {
        self._create_xt([N::PROXY_PALLET_IDX, N::PROXY_REMOVE_PROXIES], nonce)
    }

    /// Returns proxies set for current account.
    pub fn proxies<A: Into<AccountId32>>(
        &self,
        address: A,
    ) -> Result<
        Option<(
            Vec<ProxyDefinition<AccountId32, N::ProxyTypeType, u32>>,
            u128,
        )>,
    > {
        let storage_key = storage_proxy_proxies(address.into().as_ref());
        let json_req = state_get_storage(storage_key, None);
        self.client.post(json_req)?.decode_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ss58::Ss58Codec;

    #[test]
    fn proxy_type_encode() {
        let staking = ProxyType::Staking;
        staking.using_encoded(|slice| {
            assert_eq!(&slice, &b"\x03");
        });
    }

    #[test]
    fn proxies_decode() {
        let expected = (
            vec![
                ProxyDefinition {
                    delegate: AccountId32::from_ss58check(
                        "5D4YdVdApsU3Y4Qf5kCbKgFEtPKLMqoQQHcA2U45XYxwkMR6",
                    )
                    .unwrap(),
                    proxy_type: WestendProxyType::Staking,
                    delay: 0u32,
                },
                ProxyDefinition {
                    delegate: AccountId32::from_ss58check(
                        "5FTbktpmgu4oi2Nhn2qY8QuPyoJY3CDzpYFeqRcte5fZ5Yby",
                    )
                    .unwrap(),
                    proxy_type: WestendProxyType::Staking,
                    delay: 0u32,
                },
                ProxyDefinition {
                    delegate: AccountId32::from_ss58check(
                        "5FsrYox6CGyZ7rovGAmn17mJyHXf8QFN82KoGYWz6eHnRRXW",
                    )
                    .unwrap(),
                    proxy_type: WestendProxyType::Staking,
                    delay: 0u32,
                },
            ],
            1_005_350_000_000u128,
        );
        let got = hex::encode(expected.encode());
        let encoded = "0c2c1d2c0f9cd4155eafb3d912536fbe14a8679e176c89f20033291202d7e0fb930200000000962acfcc6fdb0f3a0006748ec9164392d168d4c6d06b4cc0958cf8401d7a026c0200000000a8aa7e9be09ceb0d3db02ae159bd105253e553bb6c944de7d4c35dfda4ae772c020000000080958713ea0000000000000000000000";
        assert_eq!(got, encoded);
        let encoded = hex::decode(encoded).unwrap();
        let decoded: (
            Vec<ProxyDefinition<AccountId32, WestendProxyType, u32>>,
            u128,
        ) = Decode::decode(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, expected);
    }
}
