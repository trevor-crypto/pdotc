use parity_scale_codec::{Compact, Decode, Encode};

use crate::client::{Api, Signer};
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

const STAKING_PALLET_IDX: u8 = 6;
const STAKING_BOND: CallIndex = [STAKING_PALLET_IDX, 0];
const STAKING_BOND_EXTRA: CallIndex = [STAKING_PALLET_IDX, 1];
const STAKING_UNBOND: CallIndex = [STAKING_PALLET_IDX, 2];
const STAKING_WITHDRAW_UNBONDED: CallIndex = [STAKING_PALLET_IDX, 3];
const STAKING_NOMINATE: CallIndex = [STAKING_PALLET_IDX, 5];
const STAKING_CHILL: CallIndex = [STAKING_PALLET_IDX, 6];
const STAKING_SET_CONTROLLER: CallIndex = [STAKING_PALLET_IDX, 8];
const STAKING_REBOND: CallIndex = [STAKING_PALLET_IDX, 13];

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

impl<S: Signer, Client: RpcClient> Api<'_, S, Client> {
    pub fn staking_bond(
        &self,
        controller: GenericAddress,
        amount: Balance,
        payee: RewardDestination<GenericAddress>,
    ) -> UncheckedExtrinsic<ComposedStakingBond> {
        let call = (STAKING_BOND, controller, Compact(amount), payee);
        self.create_xt(call)
    }

    pub fn staking_bond_extra(
        &self,
        amount: Balance,
    ) -> UncheckedExtrinsic<ComposedStakingBondExtra> {
        let call = (STAKING_BOND_EXTRA, Compact(amount));
        self.create_xt(call)
    }

    pub fn staking_unbond(&self, amount: Balance) -> UncheckedExtrinsic<ComposedStakingUnbond> {
        let call = (STAKING_UNBOND, Compact(amount));
        self.create_xt(call)
    }

    pub fn staking_withdraw_unbonded(
        &self,
        num_slashing_spans: u32,
    ) -> UncheckedExtrinsic<ComposedStakingWithdrawUnbonded> {
        let call = (STAKING_WITHDRAW_UNBONDED, num_slashing_spans);
        self.create_xt(call)
    }

    pub fn staking_nominate(
        &self,
        targets: Vec<GenericAddress>,
    ) -> UncheckedExtrinsic<ComposedStakingNominate> {
        let call = (STAKING_NOMINATE, targets);
        self.create_xt(call)
    }

    pub fn staking_chill(&self) -> UncheckedExtrinsic<ComposedStakingChill> {
        let call = STAKING_CHILL;
        self.create_xt(call)
    }

    pub fn staking_set_controller(
        &self,
        controller: GenericAddress,
    ) -> UncheckedExtrinsic<ComposedStakingSetController> {
        let call = (STAKING_SET_CONTROLLER, controller);
        self.create_xt(call)
    }

    pub fn staking_rebond(&self, amount: Balance) -> UncheckedExtrinsic<ComposedStakingRebond> {
        let call = (STAKING_REBOND, Compact(amount));
        self.create_xt(call)
    }
}
