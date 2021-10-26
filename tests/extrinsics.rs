use pdotc::pallets::balances::Transfer;
use pdotc::pallets::Composed;
use pdotc::*;
use sp_core::crypto::{AccountId32, Ss58Codec};

#[test]
fn create_transfer_xt() {
    let extra = GenericExtra::new(Era::Immortal, 0);
    // gen hash = 0xfe6f0f6a014986b19b9b3aff03cb4352ae9fb009b17986f46593a97e80b0d3ff
    let call = Transfer {
        dest: MultiAddress::Id(
            AccountId32::from_string("5Hq465EqSK865f4cHMgDpuKZf45ukuUshFxAPCCzmJEoBoNe").unwrap(),
        ),
        value: 1000,
    }
    .compose();

    let gen_hash: [u8; 32] =
        hex::decode("fe6f0f6a014986b19b9b3aff03cb4352ae9fb009b17986f46593a97e80b0d3ff")
            .unwrap()
            .try_into()
            .unwrap();

    let s_extra = (9120, 7, gen_hash.into(), gen_hash.into(), (), (), ());
    let raw_payload = SignedPayload::new(call.clone(), extra, s_extra);

    let _signature = raw_payload.encoded(|_payload| [0u8; 32]);

    let xt = UncheckedExtrinsic {
        signature: None,
        function: call,
    };
    println!("{}", xt.as_hex());
}
