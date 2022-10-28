use std::sync::{LazyLock, OnceLock};

use paste::paste;
use pdotc::client::{Api, ApiBuilder};
use pdotc::network::Polkadot;
use ureq::Agent;

use crate::{validate_xt, KeyStore, PDotClient};

static CLIENT: OnceLock<PDotClient<Agent>> = OnceLock::new();

static API: LazyLock<Api<KeyStore, PDotClient<Agent>, Polkadot>> =
    LazyLock::new(|| ApiBuilder::polkadot(client()).build().unwrap());

fn client() -> &'static PDotClient<Agent> {
    CLIENT.get_or_init(PDotClient::dot)
}
validate_xt!(staking_rebond(), "0x0713a10f");
validate_xt!(staking_bond_extra(), "0x0701a10f");
validate_xt!(staking_unbond(), "0x0702a10f");
validate_xt!(staking_withdraw_unbonded(), "0x070300000000");
validate_xt!(staking_chill(), "0x0706");
validate_xt!(
    balance_transfer("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x050000bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7da10f"
);
validate_xt!(
    staking_bond("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x070000bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7da10f01"
);
validate_xt!(
    staking_nominate("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x07050400bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7d"
);
validate_xt!(
    staking_set_controller("15FEzAVAanaAGtVZLEDMeRKdKipwQrTCpJd1k6k4WP4LhXgT"),
    "0x070800bbcd72f9f3d1782b57e512497ed7a1d3e2163333bb06c59723e28823798f5a7d"
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
