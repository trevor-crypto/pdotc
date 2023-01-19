use base58::{FromBase58, ToBase58};
use sp_core::crypto::{
    from_known_address_format, AccountId32, PublicError, Ss58AddressFormat,
    Ss58AddressFormatRegistry,
};
use sp_core::ByteArray;

const PREFIX: &[u8] = b"SS58PRE";

static DEFAULT_VERSION: core::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(
    from_known_address_format(Ss58AddressFormatRegistry::SubstrateAccount),
);

pub fn default_ss58_version() -> Ss58AddressFormat {
    DEFAULT_VERSION
        .load(std::sync::atomic::Ordering::Relaxed)
        .into()
}

/// Key that can be encoded to/from SS58.
///
/// See <https://docs.substrate.io/v3/advanced/ss58/>
/// for information on the codec.
pub trait Ss58Codec: Sized + AsMut<[u8]> + AsRef<[u8]> + ByteArray {
    /// A format filterer, can be used to ensure that `from_ss58check` family
    /// only decode for allowed identifiers. By default just refuses the two
    /// reserved identifiers.
    fn format_is_allowed(f: Ss58AddressFormat) -> bool {
        !f.is_reserved()
    }

    /// Some if the string is a properly encoded SS58Check address.
    fn from_ss58check(s: &str) -> Result<Self, PublicError> {
        Self::from_ss58check_with_version(s).and_then(|(r, v)| match v {
            v if !v.is_custom() => Ok(r),
            v if v == default_ss58_version() => Ok(r),
            v => Err(PublicError::UnknownSs58AddressFormat(v)),
        })
    }

    /// Some if the string is a properly encoded SS58Check address.
    fn from_ss58check_with_version(s: &str) -> Result<(Self, Ss58AddressFormat), PublicError> {
        const CHECKSUM_LEN: usize = 2;
        let body_len = Self::LEN;

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
        if !Self::format_is_allowed(format) {
            return Err(PublicError::FormatNotAllowed);
        }

        let hash = ss58hash(&data[0..body_len + prefix_len]);
        let checksum = &hash[0..CHECKSUM_LEN];
        if data[body_len + prefix_len..body_len + prefix_len + CHECKSUM_LEN] != *checksum {
            // Invalid checksum.
            return Err(PublicError::InvalidChecksum);
        }

        let result = Self::from_slice(&data[prefix_len..body_len + prefix_len])
            .map_err(|()| PublicError::BadLength)?;
        Ok((result, format))
    }

    /// Some if the string is a properly encoded SS58Check address, optionally
    /// with a derivation path following.
    fn from_string(s: &str) -> Result<Self, PublicError> {
        Self::from_string_with_version(s).and_then(|(r, v)| match v {
            v if !v.is_custom() => Ok(r),
            v if v == default_ss58_version() => Ok(r),
            v => Err(PublicError::UnknownSs58AddressFormat(v)),
        })
    }

    /// Return the ss58-check string for this key.
    fn to_ss58check_with_version(&self, version: Ss58AddressFormat) -> String {
        // We mask out the upper two bits of the ident - SS58 Prefix currently only
        // supports 14-bits
        let ident: u16 = u16::from(version) & 0b0011_1111_1111_1111;
        let mut v = match ident {
            0..=63 => vec![ident as u8],
            64..=16_383 => {
                // upper six bits of the lower byte(!)
                let first = ((ident & 0b0000_0000_1111_1100) as u8) >> 2;
                // lower two bits of the lower byte in the high pos,
                // lower bits of the upper byte in the low pos
                let second = ((ident >> 8) as u8) | ((ident & 0b0000_0000_0000_0011) as u8) << 6;
                vec![first | 0b01000000, second]
            }
            _ => unreachable!("masked out the upper two bits; qed"),
        };
        v.extend(self.as_ref());
        let r = ss58hash(&v);
        v.extend(&r[0..2]);
        v.to_base58()
    }

    /// Return the ss58-check string for this key.
    fn to_ss58check(&self) -> String {
        self.to_ss58check_with_version(default_ss58_version())
    }

    /// Some if the string is a properly encoded SS58Check address, optionally
    /// with a derivation path following.
    fn from_string_with_version(s: &str) -> Result<(Self, Ss58AddressFormat), PublicError> {
        Self::from_ss58check_with_version(s)
    }
}

fn ss58hash(data: &[u8]) -> Vec<u8> {
    use blake2::{Blake2b512, Digest};

    let mut ctx = Blake2b512::new();
    ctx.update(PREFIX);
    ctx.update(data);
    ctx.finalize().to_vec()
}

impl Ss58Codec for AccountId32 {}
