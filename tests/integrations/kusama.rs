use std::sync::{LazyLock, OnceLock};

use paste::paste;
use pdotc::client::{Api, ApiBuilder};
use pdotc::network::Kusama;
use pdotc::ss58::Ss58Codec;
use pdotc::AccountId32;
use ureq::Agent;

use crate::{get_balance, validate_xt, KeyStore, PDotClient};

static CLIENT: OnceLock<PDotClient<Agent>> = OnceLock::new();

static API: LazyLock<Api<KeyStore, PDotClient<Agent>, Kusama>> = LazyLock::new(|| {
    let client = CLIENT.get_or_init(PDotClient::ksm);
    ApiBuilder::kusama(client).build().unwrap()
});

get_balance!("GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq");

validate_xt!(staking_rebond(), "0x0613a10f");
validate_xt!(staking_bond_extra(), "0x0601a10f");
validate_xt!(staking_unbond(), "0x0602a10f");
validate_xt!(staking_withdraw_unbonded(), "0x060300000000");
validate_xt!(staking_chill(), "0x0606");
validate_xt!(
    balance_transfer("GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"),
    "0x040000c05740c342cde29bac622eba115bca3c9d2d194ef52aac55c58208cfbff8a931a10f"
);
validate_xt!(
    staking_bond("GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"),
    "0x060000c05740c342cde29bac622eba115bca3c9d2d194ef52aac55c58208cfbff8a931a10f01"
);
validate_xt!(
    staking_nominate("GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"),
    "0x06050400c05740c342cde29bac622eba115bca3c9d2d194ef52aac55c58208cfbff8a931"
);
validate_xt!(
    staking_set_controller("GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"),
    "0x060800c05740c342cde29bac622eba115bca3c9d2d194ef52aac55c58208cfbff8a931"
);
validate_xt!(
    proxy_add_proxy("GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"),
    "0x1e0100c05740c342cde29bac622eba115bca3c9d2d194ef52aac55c58208cfbff8a9310300000000"
);
validate_xt!(
    proxy_remove_proxy("GvWdZbtNY8nSFBfZ2Jr9V8hdo2F84jPdn8BBV55AQtgUsjq"),
    "0x1e0200c05740c342cde29bac622eba115bca3c9d2d194ef52aac55c58208cfbff8a9310300000000"
);
validate_xt!(proxy_remove_proxies(), "0x1e03");
