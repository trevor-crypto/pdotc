use base58::FromBase58;
use serde::Deserializer;
use sp_core::crypto::{AccountId32, PublicError, Ss58AddressFormat};
use sp_core::hashing::blake2_512;

use crate::{Balance, H256};

pub trait FromHexString {
    fn from_hex(hex: String) -> Result<Self, hex::FromHexError>
    where
        Self: Sized;
}

impl FromHexString for Vec<u8> {
    fn from_hex(hex: String) -> Result<Self, hex::FromHexError> {
        let hexstr = hex
            .trim_matches('\"')
            .to_string()
            .trim_start_matches("0x")
            .to_string();

        hex::decode(&hexstr)
    }
}

impl FromHexString for H256 {
    fn from_hex(hex: String) -> Result<Self, hex::FromHexError> {
        let vec = Vec::from_hex(hex)?;

        match vec.len() {
            32 => Ok(H256(vec.try_into().unwrap())),
            _ => Err(hex::FromHexError::InvalidStringLength),
        }
    }
}

pub fn deser_number_or_hex<'de, D>(d: D) -> Result<Balance, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(d)?;
    let num = match s.strip_prefix("0x") {
        Some(hex) => {
            // hex string
            Balance::from_str_radix(hex, 16).expect("valid Balance")
        }
        None => {
            // number
            Balance::from_str_radix(&s, 10).expect("valid Balance")
        }
    };
    Ok(num)
}

// Copied from sp-core
/// Some if the string is a properly encoded SS58Check address.
pub fn account_from_ss58check_with_version(
    s: &str,
) -> Result<(AccountId32, Ss58AddressFormat), PublicError> {
    const CHECKSUM_LEN: usize = 2;
    let body_len = 32;

    let data = s.from_base58().map_err(|_| PublicError::BadBase58)?;
    if data.len() < 2 {
        return Err(PublicError::BadLength);
    }
    let (prefix_len, ident) = match data[0] {
        0..=63 => (1, data[0] as u16),
        64..=127 => {
            // weird bit manipulation owing to the combination of LE encoding and missing
            // two bits from the left.
            // d[0] d[1] are: 01aaaaaa bbcccccc
            // they make the LE-encoded 16-bit value: aaaaaabb 00cccccc
            // so the lower byte is formed of aaaaaabb and the higher byte is 00cccccc
            let lower = (data[0] << 2) | (data[1] >> 6);
            let upper = data[1] & 0b00111111;
            (2, (lower as u16) | ((upper as u16) << 8))
        }
        _ => return Err(PublicError::InvalidPrefix),
    };
    if data.len() != prefix_len + body_len + CHECKSUM_LEN {
        return Err(PublicError::BadLength);
    }
    let format = ident.into();

    // if !Self::format_is_allowed(format) {
    //     return Err(PublicError::FormatNotAllowed);
    // }

    let hash = ss58hash(&data[0..body_len + prefix_len]);
    let checksum = &hash[0..CHECKSUM_LEN];
    if data[body_len + prefix_len..body_len + prefix_len + CHECKSUM_LEN] != *checksum {
        // Invalid checksum.
        return Err(PublicError::InvalidChecksum);
    }

    let bytes = data[prefix_len..body_len + prefix_len]
        .try_into()
        .map_err(|_| PublicError::BadLength)?;
    let result = AccountId32::new(bytes);
    Ok((result, format))
}

fn ss58hash(data: &[u8]) -> [u8; 64] {
    blake2_512(&[b"SS58PRE", data].concat())
}
