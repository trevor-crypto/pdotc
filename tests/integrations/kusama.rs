use std::lazy::{SyncLazy, SyncOnceCell};

use paste::paste;
use pdotc::client::Api;
use ureq::Agent;

use crate::{validate_xt, KeyStore, PDotClient};

static CLIENT: SyncOnceCell<PDotClient<Agent>> = SyncOnceCell::new();

static API: SyncLazy<Api<KeyStore, PDotClient<Agent>>> = SyncLazy::new(|| {
    let client = CLIENT.get_or_init(PDotClient::ksm);
    let keystore = KeyStore::default();
    Api::kusama_with_signer(&*client, keystore).unwrap()
});

validate_xt!(staking_rebond);
validate_xt!(staking_bond_extra);
validate_xt!(staking_unbond);
validate_xt!(staking_withdraw_unbonded);
validate_xt!(staking_chill);
validate_xt!(
    balance_transfer,
    "GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"
);
validate_xt!(
    staking_bond,
    "GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"
);
validate_xt!(
    staking_nominate,
    "GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"
);
validate_xt!(
    staking_set_controller,
    "GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"
);
