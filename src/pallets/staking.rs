use parity_scale_codec::{Compact, Decode, Encode};

use crate::client::{Api, Result, Signer};
use crate::network::SubstrateNetwork;
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

pub type ComposedStakingBond = (
    CallIndex,
    Compact<Balance>,
    RewardDestination<GenericAddress>,
);
pub type ComposedStakingBondExtra = (CallIndex, Compact<Balance>);
pub type ComposedStakingUnbond = (CallIndex, Compact<Balance>);
pub type ComposedStakingWithdrawUnbonded = (CallIndex, u32);
pub type ComposedStakingNominate = (CallIndex, Vec<GenericAddress>);
pub type ComposedStakingChill = CallIndex;
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
        amount: Balance,
        payee: RewardDestination<GenericAddress>,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingBond>> {
        let call = (
            [N::STAKING_PALLET_IDX, N::STAKING_BOND],
            Compact(amount),
            payee,
        );
        self._create_xt(call, nonce)
    }

    pub fn staking_bond_extra(
        &self,
        amount: Balance,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingBondExtra>> {
        let call = (
            [N::STAKING_PALLET_IDX, N::STAKING_BOND_EXTRA],
            Compact(amount),
        );
        self._create_xt(call, nonce)
    }

    pub fn staking_unbond(
        &self,
        amount: Balance,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingUnbond>> {
        let call = ([N::STAKING_PALLET_IDX, N::STAKING_UNBOND], Compact(amount));
        self._create_xt(call, nonce)
    }

    pub fn staking_withdraw_unbonded(
        &self,
        num_slashing_spans: u32,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingWithdrawUnbonded>> {
        let call = (
            [N::STAKING_PALLET_IDX, N::STAKING_WITHDRAW_UNBONDED],
            num_slashing_spans,
        );
        self._create_xt(call, nonce)
    }

    pub fn staking_nominate(
        &self,
        targets: Vec<GenericAddress>,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingNominate>> {
        let call = ([N::STAKING_PALLET_IDX, N::STAKING_NOMINATE], targets);
        self._create_xt(call, nonce)
    }

    pub fn staking_chill(
        &self,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingChill>> {
        self._create_xt([N::STAKING_PALLET_IDX, N::STAKING_CHILL], nonce)
    }

    pub fn staking_rebond(
        &self,
        amount: Balance,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedStakingRebond>> {
        let call = ([N::STAKING_PALLET_IDX, N::STAKING_REBOND], Compact(amount));
        self._create_xt(call, nonce)
    }
}
