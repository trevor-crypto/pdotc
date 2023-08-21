use crate::client::{Api, Result, Signer};
use crate::network::SubstrateNetwork;
use crate::pallets::CallIndex;
use crate::rpc::RpcClient;
use crate::UncheckedExtrinsic;

pub type ComposedJoinIdentity = (CallIndex, u64);

impl<S: Signer, Client: RpcClient, N: SubstrateNetwork> Api<'_, S, Client, N> {
    pub fn join_identity_as_key(
        &self,
        auth_id: u64,
        nonce: Option<u32>,
    ) -> Result<UncheckedExtrinsic<ComposedJoinIdentity>> {
        let call = ([N::IDENTITY_PALLET_IDX, N::IDENTITY_JOIN_AS_KEY], auth_id);
        self._create_xt(call, nonce)
    }
}
