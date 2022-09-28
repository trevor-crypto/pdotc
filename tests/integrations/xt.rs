use std::str::FromStr;

use pdotc::client::{Api, Signer};
use pdotc::network::SubstrateNetwork;
use pdotc::pallets::staking::RewardDestination;
use pdotc::rpc::RpcClient;
use pdotc::{AccountId32, MultiAddress, Ss58Codec};

pub fn balance_transfer<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> String {
    api.balance_transfer(
        MultiAddress::Id(AccountId32::from_ss58check_with_version(addr).unwrap().0),
        1000,
    )
    .unwrap()
    .call_as_hex()
}

pub fn staking_bond<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> String {
    api.staking_bond(
        MultiAddress::Id(AccountId32::from_ss58check_with_version(addr).unwrap().0),
        1000,
        RewardDestination::Stash,
    )
    .unwrap()
    .call_as_hex()
}
pub fn staking_bond_extra<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> String {
    api.staking_bond_extra(1000).unwrap().call_as_hex()
}

pub fn staking_unbond<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> String {
    api.staking_unbond(1000).unwrap().call_as_hex()
}

pub fn staking_withdraw_unbonded<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> String {
    api.staking_withdraw_unbonded(0).unwrap().call_as_hex()
}

pub fn staking_nominate<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> String {
    api.staking_nominate(vec![MultiAddress::Id(
        AccountId32::from_ss58check_with_version(addr).unwrap().0,
    )])
    .unwrap()
    .call_as_hex()
}

pub fn staking_chill<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> String {
    api.staking_chill().unwrap().call_as_hex()
}

pub fn staking_set_controller<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> String {
    api.staking_set_controller(MultiAddress::Id(
        AccountId32::from_ss58check_with_version(addr).unwrap().0,
    ))
    .unwrap()
    .call_as_hex()
}

pub fn staking_rebond<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> String {
    api.staking_rebond(1000).unwrap().call_as_hex()
}

pub fn proxy_add_proxy<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> String {
    api.add_proxy(
        FromStr::from_str(addr).unwrap(),
        FromStr::from_str("Staking").unwrap(),
        0,
    )
    .unwrap()
    .call_as_hex()
}

pub fn proxy_remove_proxy<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
    addr: &str,
) -> String {
    api.remove_proxy(
        FromStr::from_str(addr).unwrap(),
        FromStr::from_str("Staking").unwrap(),
        0,
    )
    .unwrap()
    .call_as_hex()
}

pub fn proxy_remove_proxies<S: Signer, Client: RpcClient, N: SubstrateNetwork>(
    api: &Api<S, Client, N>,
) -> String {
    api.remove_proxies().unwrap().call_as_hex()
}
