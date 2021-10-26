use parity_scale_codec::{Compact, Decode, Encode};

use crate::client::{Api, Result, Signer};
use crate::pallets::{Composed, BALANCES_TRANSFER};
use crate::rpc::RpcClient;
use crate::{MultiAddress, UncheckedExtrinsic};

/// Pallet: Balances
/// Function: transfer
#[derive(Debug, Clone, Encode, Decode)]
pub struct Transfer {
    /// Destination address pubkey
    pub dest: MultiAddress,
    /// Amount of funds to transfer
    pub value: u128,
}

type ComposedTransfer = ([u8; 2], MultiAddress, Compact<u128>);

impl Composed for Transfer {
    type ComposedType = ComposedTransfer;

    fn compose(&self) -> Self::ComposedType {
        (BALANCES_TRANSFER, self.dest.clone(), Compact(self.value))
    }
}

impl<S: Signer, Client: RpcClient> Api<S, Client> {
    pub fn transfer_xt(
        &self,
        to: MultiAddress,
        amount: u128,
    ) -> Result<UncheckedExtrinsic<ComposedTransfer>> {
        let call = Transfer {
            dest: to,
            value: amount,
        };

        let xt = self.create_xt(call);
        Ok(xt)
    }
}
