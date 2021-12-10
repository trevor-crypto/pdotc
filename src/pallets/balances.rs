use parity_scale_codec::Compact;

use crate::client::{Api, Result, Signer};
use crate::pallets::{CallIndex, NetworkPallets};
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

pub type ComposedTransfer = (CallIndex, GenericAddress, Compact<Balance>);

impl<S: Signer, Client: RpcClient, Network: NetworkPallets> Api<'_, S, Client, Network> {
    const BALANCES_TRANSFER: CallIndex = [Network::BALANCE_PALLET_IDX, 0];

    pub fn balance_transfer(
        &self,
        to: GenericAddress,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedTransfer>> {
        let call = (Self::BALANCES_TRANSFER, to, Compact(amount));
        self.create_xt(call)
    }
}
