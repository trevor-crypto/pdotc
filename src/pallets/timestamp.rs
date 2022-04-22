use parity_scale_codec::{Compact, Decode};

use crate::pallets::CallIndex;
use crate::UncheckedExtrinsic;

pub type Timestamp = (CallIndex, Compact<u64>);

/// Expects a Timestamp extrinsic and extracts the Unix time from it
pub fn decode_timestamp(xt_str: &str) -> Option<u64> {
    let data = hex::decode(xt_str.trim_start_matches("0x")).ok()?;
    let timestamp_xt: UncheckedExtrinsic<Timestamp> =
        UncheckedExtrinsic::decode(&mut data.as_slice()).ok()?;
    Some(timestamp_xt.function.1 .0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_decode_timestamp() {
        assert_eq!(
            decode_timestamp("0x280403000b83f6d54f8001").unwrap(),
            1650606864003
        );
        assert_eq!(
            decode_timestamp("280403000b83f6d54f8001").unwrap(),
            1650606864003
        );
    }
}
