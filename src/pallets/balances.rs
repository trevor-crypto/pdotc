use parity_scale_codec::Compact;

use crate::client::{Api, Result, Signer};
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

const BALANCES_PALLET_IDX: u8 = 4;
const BALANCES_TRANSFER: CallIndex = [BALANCES_PALLET_IDX, 0];

pub type ComposedTransfer = (CallIndex, GenericAddress, Compact<Balance>);

impl<S: Signer, Client: RpcClient> Api<'_, S, Client> {
    pub fn balance_transfer(
        &self,
        to: GenericAddress,
        amount: Balance,
    ) -> Result<UncheckedExtrinsic<ComposedTransfer>> {
        let call = (BALANCES_TRANSFER, to, Compact(amount));
        self.create_xt(call)
    }
}
