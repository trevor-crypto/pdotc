use parity_scale_codec::{Compact, Decode, Encode};

use crate::client::{Api, Result, Signer};
use crate::pallets::{CallIndex, NetworkPallets};
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

const BOND: u8 = 0;
const BOND_EXTRA: u8 = 1;
const UNBOND: u8 = 2;
const WITHDRAW_UNBONDED: u8 = 3;
const NOMINATE: u8 = 5;
const CHILL: u8 = 6;
const SET_CONTROLLER: u8 = 8;
const REBOND: u8 = 19;

pub type ComposedStakingBond = (
    CallIndex,
    GenericAddress,
    Compact<Balance>,
    RewardDestination<GenericAddress>,
);
pub type ComposedStakingBondExtra = (CallIndex, Compact<Balance>);
pub type ComposedStakingUnbond = (CallIndex, Compact<Balance>);
pub type ComposedStakingWithdrawUnbonded = (CallIndex, u32);
pub type ComposedStakingNominate = (CallIndex, Vec<GenericAddress>);
pub type ComposedStakingChill = CallIndex;
pub type ComposedStakingSetController = (CallIndex, GenericAddress);
pub type ComposedStakingRebond = (CallIndex, Compact<Balance>);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode)]
pub enum RewardDestination<Account> {
    Staked,
    Stash,
    Controller,
    Account(Account),
    None,
}

impl<S: Signer, Client: RpcClient, Network: NetworkPallets> Api<'_, S, Client, Network> {
    const fn staking_call(func_idx: u8) -> CallIndex {
        [Network::STAKING_PALLET_IDX, func_idx]
    }

    pub fn staking_bond(
        &self,
        controller: GenericAddress,
        amount: Balance,
        payee: RewardDestination<GenericAddress>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingBond>> {
        let call = (Self::staking_call(BOND), controller, Compact(amount), payee);
        self.create_xt(call)
    }

    pub fn staking_bond_extra(
        &self,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedStakingBondExtra>> {
        let call = (Self::staking_call(BOND_EXTRA), Compact(amount));
        self.create_xt(call)
    }

    pub fn staking_unbond(
        &self,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedStakingUnbond>> {
        let call = (Self::staking_call(UNBOND), Compact(amount));
        self.create_xt(call)
    }

    pub fn staking_withdraw_unbonded(
        &self,
        num_slashing_spans: u32,
    ) -> Result<UncheckedExtrinsic<ComposedStakingWithdrawUnbonded>> {
        let call = (Self::staking_call(WITHDRAW_UNBONDED), num_slashing_spans);
        self.create_xt(call)
    }

    pub fn staking_nominate(
        &self,
        targets: Vec<GenericAddress>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingNominate>> {
        let call = (Self::staking_call(NOMINATE), targets);
        self.create_xt(call)
    }

    pub fn staking_chill(&self) -> Result<UncheckedExtrinsic<ComposedStakingChill>> {
        let call = Self::staking_call(CHILL);
        self.create_xt(call)
    }

    pub fn staking_set_controller(
        &self,
        controller: GenericAddress,
    ) -> Result<UncheckedExtrinsic<ComposedStakingSetController>> {
        let call = (Self::staking_call(SET_CONTROLLER), controller);
        self.create_xt(call)
    }

    pub fn staking_rebond(
        &self,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedStakingRebond>> {
        let call = (Self::staking_call(REBOND), Compact(amount));
        self.create_xt(call)
    }
}
