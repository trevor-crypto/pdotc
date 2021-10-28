use parity_scale_codec::{Compact, Decode, Encode};

use crate::client::{Api, Result, Signer};
use crate::pallets::{CallIndex, Composed, BALANCES_TRANSFER};
use crate::rpc::RpcClient;
use crate::{GenericAddress, UncheckedExtrinsic};

/// Pallet: Balances
/// Function: transfer
#[derive(Debug, Clone, Encode, Decode)]
pub struct Transfer {
    /// Destination address pubkey
    pub dest: GenericAddress,
    /// Amount of funds to transfer
    pub value: u128,
}

type ComposedTransfer = (CallIndex, GenericAddress, Compact<u128>);

impl Composed for Transfer {
    type ComposedType = ComposedTransfer;

    fn compose(&self) -> Self::ComposedType {
        (BALANCES_TRANSFER, self.dest.clone(), Compact(self.value))
    }
}

impl<S: Signer, Client: RpcClient> Api<'_, S, Client> {
    pub fn balance_transfer(
        &self,
        to: GenericAddress,
        amount: u128,
    ) -> Result<UncheckedExtrinsic<ComposedTransfer>> {
        let call = Transfer {
            dest: to,
            value: amount,
        };
        let nonce = self.nonce()?;
        let xt = self.create_xt(call, nonce);
        Ok(xt)
    }
}
