#![allow(clippy::type_complexity)]
use std::str::FromStr;

use pdotc::client::{Api, Signer};
use pdotc::network::SubstrateNetwork;
use pdotc::pallets::balances::ComposedTransfer;
use pdotc::pallets::staking::{
    ComposedStakingBond, ComposedStakingBondExtra, ComposedStakingChill, ComposedStakingNominate,
    ComposedStakingRebond, ComposedStakingSetController, ComposedStakingUnbond,
    ComposedStakingWithdrawUnbonded, RewardDestination,
};
use pdotc::rpc::RpcClient;
use pdotc::{AccountId32, MultiAddress, Ss58Codec, UncheckedExtrinsic};

pub fn balance_transfer<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> UncheckedExtrinsic<ComposedTransfer> {
    api.balance_transfer(
        MultiAddress::Id(AccountId32::from_ss58check_with_version(addr).unwrap().0),
        1000,
    )
    .unwrap()
}

pub fn staking_bond<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> UncheckedExtrinsic<ComposedStakingBond> {
    api.staking_bond(
        MultiAddress::Id(AccountId32::from_ss58check_with_version(addr).unwrap().0),
        1000,
        RewardDestination::Stash,
    )
    .unwrap()
}
pub fn staking_bond_extra<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> UncheckedExtrinsic<ComposedStakingBondExtra> {
    api.staking_bond_extra(1000).unwrap()
}

pub fn staking_unbond<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> UncheckedExtrinsic<ComposedStakingUnbond> {
    api.staking_unbond(1000).unwrap()
}

pub fn staking_withdraw_unbonded<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> UncheckedExtrinsic<ComposedStakingWithdrawUnbonded> {
    api.staking_withdraw_unbonded(0).unwrap()
}

pub fn staking_nominate<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> UncheckedExtrinsic<ComposedStakingNominate> {
    api.staking_nominate(vec![MultiAddress::Id(
        AccountId32::from_ss58check_with_version(addr).unwrap().0,
    )])
    .unwrap()
}

pub fn staking_chill<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> UncheckedExtrinsic<ComposedStakingChill> {
    api.staking_chill().unwrap()
}

pub fn staking_set_controller<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> UncheckedExtrinsic<ComposedStakingSetController> {
    api.staking_set_controller(MultiAddress::Id(
        AccountId32::from_ss58check_with_version(addr).unwrap().0,
    ))
    .unwrap()
}

pub fn staking_rebond<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> UncheckedExtrinsic<ComposedStakingRebond> {
    api.staking_rebond(1000).unwrap()
}

pub fn proxy_add_proxy<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> UncheckedExtrinsic<(
    [u8; 2],
    <N as SubstrateNetwork>::ProxyDelegateType,
    <N as SubstrateNetwork>::ProxyTypeType,
    u32,
)> {
    api.add_proxy(
        FromStr::from_str(addr).unwrap(),
        FromStr::from_str("Staking").unwrap(),
        0,
    )
    .unwrap()
}

pub fn proxy_remove_proxy<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> UncheckedExtrinsic<(
    [u8; 2],
    <N as SubstrateNetwork>::ProxyDelegateType,
    <N as SubstrateNetwork>::ProxyTypeType,
    u32,
)> {
    api.remove_proxy(
        FromStr::from_str(addr).unwrap(),
        FromStr::from_str("Staking").unwrap(),
        0,
    )
    .unwrap()
}

pub fn proxy_remove_proxies<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> UncheckedExtrinsic<[u8; 2]> {
    api.remove_proxies().unwrap()
}
