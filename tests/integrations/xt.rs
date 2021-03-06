use pdotc::client::{Api, Signer};
use pdotc::pallets::staking::RewardDestination;
use pdotc::rpc::RpcClient;
use pdotc::{account_from_ss58check_with_version, MultiAddress};

pub fn balance_transfer<S: Signer, Client: RpcClient>(api: &Api<S, Client>, addr: &str) -> String {
    api.balance_transfer(
        MultiAddress::Id(account_from_ss58check_with_version(addr).unwrap().0),
        1000,
    )
    .unwrap()
    .as_hex()
}

pub fn staking_bond<S: Signer, Client: RpcClient>(api: &Api<S, Client>, addr: &str) -> String {
    api.staking_bond(
        MultiAddress::Id(account_from_ss58check_with_version(addr).unwrap().0),
        1000,
        RewardDestination::Stash,
    )
    .unwrap()
    .as_hex()
}
pub fn staking_bond_extra<S: Signer, Client: RpcClient>(api: &Api<S, Client>) -> String {
    api.staking_bond_extra(1000).unwrap().as_hex()
}

pub fn staking_unbond<S: Signer, Client: RpcClient>(api: &Api<S, Client>) -> String {
    api.staking_unbond(1000).unwrap().as_hex()
}

pub fn staking_withdraw_unbonded<S: Signer, Client: RpcClient>(api: &Api<S, Client>) -> String {
    api.staking_withdraw_unbonded(0).unwrap().as_hex()
}

pub fn staking_nominate<S: Signer, Client: RpcClient>(api: &Api<S, Client>, addr: &str) -> String {
    api.staking_nominate(vec![MultiAddress::Id(
        account_from_ss58check_with_version(addr).unwrap().0,
    )])
    .unwrap()
    .as_hex()
}

pub fn staking_chill<S: Signer, Client: RpcClient>(api: &Api<S, Client>) -> String {
    api.staking_chill().unwrap().as_hex()
}

pub fn staking_set_controller<S: Signer, Client: RpcClient>(
    api: &Api<S, Client>,
    addr: &str,
) -> String {
    api.staking_set_controller(MultiAddress::Id(
        account_from_ss58check_with_version(addr).unwrap().0,
    ))
    .unwrap()
    .as_hex()
}

pub fn staking_rebond<S: Signer, Client: RpcClient>(api: &Api<S, Client>) -> String {
    api.staking_rebond(1000).unwrap().as_hex()
}
