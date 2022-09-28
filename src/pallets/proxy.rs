use std::fmt::Debug;
use std::str::FromStr;

use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::AccountId32;

use crate::client::{Api, Result, Signer};
use crate::network::SubstrateNetwork;
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::{GenericAddress, UncheckedExtrinsic};

pub type ComposedProxyAddProxy = (CallIndex, AccountId32, ProxyType, u32);
pub type ComposedProxyRemoveProxy = (CallIndex, AccountId32, ProxyType, u32);

pub type WndComposedProxyAddProxy = (CallIndex, GenericAddress, WestendProxyType, u32);
pub type WndComposedProxyRemoveProxy = (CallIndex, GenericAddress, WestendProxyType, u32);

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

#[allow(clippy::type_complexity)]
impl<S: Signer, C: RpcClient, N: SubstrateNetwork> Api<'_, S, C, N> {
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
    ) -> Result<UncheckedExtrinsic<(CallIndex, N::ProxyDelegateType, N::ProxyTypeType, u32)>> {
        let call = (
            [N::PROXY_PALLET_IDX, N::PROXY_ADD_PROXY],
            delegate,
            proxy_type,
            delay,
        );
        self.create_xt(call)
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
    ) -> Result<UncheckedExtrinsic<(CallIndex, N::ProxyDelegateType, N::ProxyTypeType, u32)>> {
        let call = (
            [N::PROXY_PALLET_IDX, N::PROXY_REMOVE_PROXY],
            delegate,
            proxy_type,
            delay,
        );
        self.create_xt(call)
    }

    /// Unregister all proxy accounts for the sender.
    pub fn remove_proxies(&self) -> Result<UncheckedExtrinsic<ComposedProxyRemoveProxies>> {
        self.create_xt([N::PROXY_PALLET_IDX, N::PROXY_REMOVE_PROXIES])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proxy_type_encode() {
        let staking = ProxyType::Staking;
        staking.using_encoded(|slice| {
            assert_eq!(&slice, &b"\x03");
        });
    }
}
