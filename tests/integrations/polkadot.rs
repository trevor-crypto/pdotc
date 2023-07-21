use std::sync::{LazyLock, OnceLock};

use paste::paste;
use pdotc::client::{Api, ApiBuilder};
use pdotc::network::Polkadot;
use pdotc::ss58::Ss58Codec;
use pdotc::AccountId32;
use ureq::Agent;

use crate::{get_balance, validate_xt, KeyStore, PDotClient};

static CLIENT: OnceLock<PDotClient<Agent>> = OnceLock::new();

static API: LazyLock<Api<KeyStore, PDotClient<Agent>, Polkadot>> = LazyLock::new(|| {
    let client = CLIENT.get_or_init(PDotClient::dot);
    ApiBuilder::polkadot(client).build().unwrap()
});

get_balance!("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT");

validate_xt!(staking_rebond(), "0x0713a10f");
validate_xt!(staking_bond_extra(), "0x0701a10f");
validate_xt!(staking_unbond(), "0x0702a10f");
validate_xt!(staking_withdraw_unbonded(), "0x070300000000");
validate_xt!(staking_chill(), "0x0706");
validate_xt!(
    balance_transfer("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x050000bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7da10f"
);
validate_xt!(staking_bond(), "0x0700a10f01");
validate_xt!(
    staking_nominate("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x07050400bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7d"
);
validate_xt!(
    proxy_add_proxy("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x1d0100bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7d0300000000"
);
validate_xt!(
    proxy_remove_proxy("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x1d0200bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7d0300000000"
);
validate_xt!(proxy_remove_proxies(), "0x1d03");
