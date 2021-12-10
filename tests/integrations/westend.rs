use paste::paste;
use pdotc::client::Api;

use crate::{KeyStore, PDotClient};

macro_rules! cmp_xt {
    ($call:ident, $expect: literal $(,$args:literal),*) => {
        paste! {
            #[test]
            fn  [<test_ $call>]() {
                let client = PDotClient::wnd();
                let keystore = KeyStore::default();
                let api = Api::westend_with_signer(&client, keystore).unwrap();

                let xt = $crate::xt::$call(&api, $($args)*);
                assert_eq!(xt, $expect);
            }
         }
    };
}

cmp_xt!(staking_rebond, "0xad01840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02562b451662af582c67e791446af4cb442d2373caed65ffe7f7fdfe5be4cb1bb946fcf31703159e1ba13fda097040282d13240e7b20711e668c29b3d8ac3a6540010000000613a10f");
cmp_xt!(staking_bond_extra, "0xad01840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf0206532cb3a071945d0e33bc88476477571e873323503159be054f664784f3e7c96ff6b3df5894839f3411c7953cc242644c20f6b46d4cce6033d85c29494380b5010000000601a10f");
cmp_xt!(staking_unbond, "0xad01840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf0235417d4c5405ca5ea5d405035f213e26597a0bd155f95e1fb89d86f0f0388de33f5be5967cb70246ad158f23eba7de31aad4884ff60ede97ace08a8d1115732d010000000602a10f");
cmp_xt!(staking_withdraw_unbonded, "0xb501840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf027c1da7a27de23dfd47a6eac857d231a9072e76d2fe1d1a9ded0a611177d836da047c11f7fba127e1ef15ffb342cafd1374fb464fdb0484d4c43fb28e93adffbb01000000070300000000");
cmp_xt!(staking_chill, "0xa501840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02d851f7f9b1afd6fefe265b4edb3956456bb9dc9e69d17683247f6a4f36a496230127b55ffa5a143063953145bc2a04dfa2d74708cdb1df77e057f9207369be32000000000606");
cmp_xt!(
    balance_transfer, "0x3102840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf029a3bba780f832cb8d5e29420c9b2cf76512c5b887c37c7bfbf6dbf5e85a2cfa408f81284281d1b65f86f149ec7264c55f232bea9f06eb824c63708de6cf786a900000000040000ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0fa10f",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
cmp_xt!(
    staking_bond, "0x3502840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02b748cf132efcdfe3cce21216ff5635551a6349c0f8471bd6a94ded369496fbc95f3685c9caa2ac1465084914b12264bd07d9bfe97b0e736ff014e45144fff8f900000000060000ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0fa10f01",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
cmp_xt!(
    staking_nominate, "0x2d02840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf028fceeff75f8a7b7e89be5fdbcc2f54e76cc446299db5e6752d5abef9255bd0b3699ed1cfa76eaa1f32a07262d1c951c98476d5f6d8c453638327b883a592cb070100000006050400ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
cmp_xt!(
    staking_set_controller, "0x2902840023c0b6f69f5aff6c91972c64d1f7d1d22c78f825796553f4a261f514712dafaf02a5bddd64877d3e3d3297f2aed0df35545430c8c6578722a1641408ba91cab66a6635838b98218786ecfe536617d888675d16ca833fbc75eadff2aabe5689aaef00000000060800ff0011afc404c2f8c72ec8bcdeb64d6367822bf3a205a9ac4c1b17ffa75c3f0f",
    "5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe"
);
