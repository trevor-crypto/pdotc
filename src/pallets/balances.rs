use parity_scale_codec::Compact;

use crate::client::{Api, Network, Result, Signer};
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

pub type ComposedTransfer = (CallIndex, GenericAddress, Compact<Balance>);

const DOT_BALANCE_PALLET_IDX: u8 = 5;
const WND_BALANCE_PALLET_IDX: u8 = 4;
const KSM_BALANCE_PALLET_IDX: u8 = 4;

impl<S: Signer, Client: RpcClient> Api<'_, S, Client> {
    fn balances_call(&self) -> CallIndex {
        let pallet_idx = match self.network {
            Network::Polkadot => DOT_BALANCE_PALLET_IDX,
            Network::Westend => WND_BALANCE_PALLET_IDX,
            Network::Kusama => KSM_BALANCE_PALLET_IDX,
        };
        [pallet_idx, 0]
    }

    pub fn balance_transfer(
        &self,
        to: GenericAddress,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedTransfer>> {
        let call = (self.balances_call(), to, Compact(amount));
        self.create_xt(call)
    }
}
