use paste::paste;
use pdotc::client::Api;

use crate::{KeyStore, PDotClient};

macro_rules! cmp_xt {
    ($call:ident, $expect: literal $(,$args:literal),*) => {
        paste! {
            #[test]
            fn  [<test_ $call>]() {
                let client = PDotClient::dot();
                let keystore = KeyStore::default();
                let api = Api::polkadot_with_signer(&client, keystore).unwrap();

                let xt = $crate::xt::$call(&api, $($args)*);
                assert_eq!(xt, $expect);
            }
         }
    };
}

cmp_xt!(staking_rebond, "0xad01840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf0212324c8b170ef2bcfa0feb39d25a5827916e738ddeef675eba9d3d164f39c35b3e71fb074d63e70b874f8f23bec84041d41ca7751a1d854eabdd0dd12a2f6751010000000713a10f");
cmp_xt!(staking_bond_extra, "0xad01840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02fff147d31f5478109b4d58a67f87c5fde938089350afb983320c6b5c6abce8f116976ab54ddeb74f9052b155ae82944c386b99e6629d52cffb33ba2b7f583a17000000000701a10f");
cmp_xt!(staking_unbond, "0xad01840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02a0af62e85783b150a9a34dadb6b0d7df409eac9ae64f14952f2132e8dd1dcc9a145294baab86dd591ffc9170d95370d11ab60d8f491cc0ce05fd858fb6d617ee000000000702a10f");
cmp_xt!(staking_withdraw_unbonded, "0xb501840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf027c1da7a27de23dfd47a6eac857d231a9072e76d2fe1d1a9ded0a611177d836da047c11f7fba127e1ef15ffb342cafd1374fb464fdb0484d4c43fb28e93adffbb01000000070300000000");
cmp_xt!(staking_chill, "0xa501840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02baf2bcfaae48dfb88b3fca08f99a6b04b32450020e88d8e747f22aa90c15fb167481a952a09ed576f53e9f009b4573906c1674bf225ab181b0546b387ee12184000000000706");
cmp_xt!(
    balance_transfer, "0x3102840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02980297c9671f12fdbac032f0b874bb257a0214eee15835e0300034b0783a5d986e32c6192ce340a6e87a6d103a23a349c347c2b2b54ac86c2d510c7c611d608e01000000050000ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0fa10f",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
cmp_xt!(
    staking_bond, "0x3502840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02a495f88e49c66fb503c83d59ccc94d185bffdb1357172e2d57acd2b416d690f91e69b91cf0bb9d3e1fafb3660afc44df7c692861c4f6d31702e8ace2b059cf7000000000070000ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0fa10f01",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
cmp_xt!(
    staking_nominate, "0x2d02840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02209269e1460af103d95fbcf4caea6fe4d1cbd20c7636ef788840850f2b3cb39272f8873cda79a2343cf0b467eb192a63576ef8beedee31352afdd53297115e930000000007050400ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
cmp_xt!(
    staking_set_controller, "0x2902840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02d6fe3830c1c272d6be975ce8d35ab0940ac7c4849976edb2422b283feb3d8c0a19cbb6a32802445bf5a21e406f7c6c20bf32026be07cf303e61690db711bf1d900000000070800ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
