use parity_scale_codec::Compact;

use crate::client::{Api, Result, Signer};
use crate::network::SubstrateNetwork;
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::{Balance, GenericAddress, UncheckedExtrinsic};

pub type ComposedTransfer = (CallIndex, GenericAddress, Compact<Balance>);

impl<S: Signer, Client: RpcClient, N: SubstrateNetwork> Api<'_, S, Client, N> {
    pub fn balance_transfer(
        &self,
        to: GenericAddress,
        amount: Balance,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedTransfer>> {
        let call = (
            [N::BALANCE_PALLET_IDX, N::BALANCE_TRANSFER],
            to,
            Compact(amount),
        );
        self._create_xt(call, nonce)
    }
}
