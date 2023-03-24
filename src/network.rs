use std::str::FromStr;

use parity_scale_codec::{Decode, Encode};

use crate::pallets::proxy::{ProxyType, WestendProxyType};
use crate::GenericAddress;

pub trait SubstrateNetwork: Clone + Copy + 'static {
    // Network name
    const NAME: &'static str;

    // Balance Pallet
    const BALANCE_PALLET_IDX: u8;
    const BALANCE_TRANSFER: u8 = 0;

    // Staking Pallet
    const STAKING_PALLET_IDX: u8;
    const STAKING_BOND: u8 = 0;
    const STAKING_BOND_EXTRA: u8 = 1;
    const STAKING_UNBOND: u8 = 2;
    const STAKING_WITHDRAW_UNBONDED: u8 = 3;
    const STAKING_NOMINATE: u8 = 5;
    const STAKING_CHILL: u8 = 6;
    const STAKING_SET_CONTROLLER: u8 = 8;
    const STAKING_REBOND: u8 = 19;

    // Proxy Pallet
    const PROXY_PALLET_IDX: u8;
    const PROXY_ADD_PROXY: u8 = 1;
    const PROXY_REMOVE_PROXY: u8 = 2;
    const PROXY_REMOVE_PROXIES: u8 = 3;
    type ProxyDelegateType: Encode + Decode + Clone + FromStr<Err = &'static str>;
    type ProxyTypeType: Encode + Decode + Clone + FromStr<Err = &'static str>;
}

#[derive(Debug, Copy, Clone)]
pub struct Polkadot;
#[derive(Debug, Copy, Clone)]
pub struct Westend;
#[derive(Debug, Copy, Clone)]
pub struct Kusama;
#[derive(Debug, Copy, Clone)]
pub struct Polymesh;

impl SubstrateNetwork for Polkadot {
    const NAME: &'static str = "polkadot";
    const BALANCE_PALLET_IDX: u8 = 5;
    const STAKING_PALLET_IDX: u8 = 7;
    const PROXY_PALLET_IDX: u8 = 29;
    type ProxyDelegateType = GenericAddress;
    type ProxyTypeType = ProxyType;
}

impl SubstrateNetwork for Westend {
    const NAME: &'static str = "westend";
    const BALANCE_PALLET_IDX: u8 = 4;
    const STAKING_PALLET_IDX: u8 = 6;
    const PROXY_PALLET_IDX: u8 = 22;
    type ProxyDelegateType = GenericAddress;
    type ProxyTypeType = WestendProxyType;
}

impl SubstrateNetwork for Kusama {
    const NAME: &'static str = "kusama";
    const BALANCE_PALLET_IDX: u8 = 4;
    const STAKING_PALLET_IDX: u8 = 6;
    const PROXY_PALLET_IDX: u8 = 30;
    type ProxyDelegateType = GenericAddress;
    type ProxyTypeType = ProxyType;
}

impl SubstrateNetwork for Polymesh {
    const NAME: &'static str = "polymesh";
    const BALANCE_PALLET_IDX: u8 = 5;
    const STAKING_PALLET_IDX: u8 = 17;
    const PROXY_PALLET_IDX: u8 = 0;
    type ProxyDelegateType = GenericAddress;
    type ProxyTypeType = ProxyType;
}
