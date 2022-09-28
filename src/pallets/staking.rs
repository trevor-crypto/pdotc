use parity_scale_codec::{Compact, Decode, Encode};

use crate::client::{Api, Result, Signer};
use crate::network::SubstrateNetwork;
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

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

impl<S: Signer, Client: RpcClient, N: SubstrateNetwork> Api<'_, S, Client, N> {
    pub fn staking_bond(
        &self,
        controller: GenericAddress,
        amount: Balance,
        payee: RewardDestination<GenericAddress>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingBond>> {
        let call = (
            [N::STAKING_PALLET_IDX, N::STAKING_BOND],
            controller,
            Compact(amount),
            payee,
        );
        self.create_xt(call)
    }

    pub fn staking_bond_extra(
        &self,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedStakingBondExtra>> {
        let call = (
            [N::STAKING_PALLET_IDX, N::STAKING_BOND_EXTRA],
            Compact(amount),
        );
        self.create_xt(call)
    }

    pub fn staking_unbond(
        &self,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedStakingUnbond>> {
        let call = ([N::STAKING_PALLET_IDX, N::STAKING_UNBOND], Compact(amount));
        self.create_xt(call)
    }

    pub fn staking_withdraw_unbonded(
        &self,
        num_slashing_spans: u32,
    ) -> Result<UncheckedExtrinsic<ComposedStakingWithdrawUnbonded>> {
        let call = (
            [N::STAKING_PALLET_IDX, N::STAKING_WITHDRAW_UNBONDED],
            num_slashing_spans,
        );
        self.create_xt(call)
    }

    pub fn staking_nominate(
        &self,
        targets: Vec<GenericAddress>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingNominate>> {
        let call = ([N::STAKING_PALLET_IDX, N::STAKING_NOMINATE], targets);
        self.create_xt(call)
    }

    pub fn staking_chill(&self) -> Result<UncheckedExtrinsic<ComposedStakingChill>> {
        self.create_xt([N::STAKING_PALLET_IDX, N::STAKING_CHILL])
    }

    pub fn staking_set_controller(
        &self,
        controller: GenericAddress,
    ) -> Result<UncheckedExtrinsic<ComposedStakingSetController>> {
        let call = (
            [N::STAKING_PALLET_IDX, N::STAKING_SET_CONTROLLER],
            controller,
        );
        self.create_xt(call)
    }

    pub fn staking_rebond(
        &self,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedStakingRebond>> {
        let call = ([N::STAKING_PALLET_IDX, N::STAKING_REBOND], Compact(amount));
        self.create_xt(call)
    }
}
