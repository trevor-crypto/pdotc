use serde::Deserializer;

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
    let num = if s.starts_with("0x") {
        // hex string
        Balance::from_str_radix(&s[2..], 16).expect("valid Balance")
    } else {
        // number
        Balance::from_str_radix(&s, 10).expect("valid Balance")
    };
    Ok(num)
}
