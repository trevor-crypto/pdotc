use std::sync::{LazyLock, OnceLock};

use paste::paste;
use pdotc::client::{Api, ApiBuilder};
use pdotc::network::Westend;
use ureq::Agent;

use crate::{validate_xt, KeyStore, PDotClient};

static CLIENT: OnceLock<PDotClient<Agent>> = OnceLock::new();

static API: LazyLock<Api<KeyStore, PDotClient<Agent>, Westend>> = LazyLock::new(|| {
    let client = CLIENT.get_or_init(PDotClient::wnd);
    ApiBuilder::westend(client).build().unwrap()
});

validate_xt!(staking_rebond(), "0x0613a10f");
validate_xt!(staking_bond_extra(), "0x0601a10f");
validate_xt!(staking_unbond(), "0x0602a10f");
validate_xt!(staking_withdraw_unbonded(), "0x060300000000");
validate_xt!(staking_chill(), "0x0606");
validate_xt!(
    balance_transfer("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"),
    "0x040000ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0fa10f"
);
validate_xt!(
    staking_bond("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"),
    "0x060000ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0fa10f01"
);
validate_xt!(
    staking_nominate("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"),
    "0x06050400ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f"
);
validate_xt!(
    staking_set_controller("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"),
    "0x060800ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f"
);
validate_xt!(
    proxy_add_proxy("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"),
    "0x160100ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f0200000000"
);
validate_xt!(
    proxy_remove_proxy("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"),
    "0x160200ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f0200000000"
);
validate_xt!(proxy_remove_proxies(), "0x1603");
