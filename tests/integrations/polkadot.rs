use std::lazy::{SyncLazy, SyncOnceCell};

use paste::paste;
use pdotc::client::Api;
use ureq::Agent;

use crate::{validate_xt, KeyStore, PDotClient};

static CLIENT: SyncOnceCell<PDotClient<Agent>> = SyncOnceCell::new();

static API: SyncLazy<Api<KeyStore, PDotClient<Agent>>> = SyncLazy::new(|| {
    let client = CLIENT.get_or_init(PDotClient::dot);
    let keystore = KeyStore::default();
    Api::polkadot_with_signer(&*client, keystore).unwrap()
});

validate_xt!(staking_rebond);
validate_xt!(staking_bond_extra);
validate_xt!(staking_unbond);
validate_xt!(staking_withdraw_unbonded);
validate_xt!(staking_chill);
validate_xt!(
    balance_transfer,
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
validate_xt!(
    staking_bond,
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
validate_xt!(
    staking_nominate,
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
validate_xt!(
    staking_set_controller,
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
