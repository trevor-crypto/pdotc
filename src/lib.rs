use parity_scale_codec::{Compact, Decode, Encode, Error, Input};
use serde::{Deserialize, Serialize};
pub use sp_core::blake2_256;
pub use sp_core::crypto::{AccountId32, Ss58Codec};
use sp_core::ecdsa::{Public, Signature};

pub mod client;
pub mod pallets;
pub mod rpc;
pub mod utils;

#[derive(Debug, Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct H256(pub [u8; 32]);

pub type GenericAddress = MultiAddress<AccountId32, ()>;

#[derive(Clone, Debug, Decode, Encode, PartialEq)]
pub enum MultiAddress<AccountId, AccountIndex> {
    /// It's an account ID (pubkey).
    Id(AccountId),
    /// It's an account index.
    Index(#[codec(compact)] AccountIndex),
    /// It's some arbitrary raw bytes.
    Raw(Vec<u8>),
    /// It's a 32 byte representation.
    Address32([u8; 32]),
    /// Its a 20 byte representation.
    Address20([u8; 20]),
}

impl From<Public> for GenericAddress {
    fn from(p: Public) -> Self {
        let acct = public_into_account(p);
        MultiAddress::Id(acct)
    }
}

pub fn public_into_account(p: Public) -> AccountId32 {
    let hash = blake2_256(&p.0);
    hash.into()
}

#[derive(Clone, Debug, Decode, Encode, PartialEq)]
pub enum MultiSignature {
    /// An ECDSA/SECP256k1 signature.
    #[codec(index = 2)]
    Ecdsa(Signature),
}

/// runtime spec verion, transaction version, genesis hash, genesis hash or
/// current hash, ...others
pub type SignedExtra = (u32, u32, H256, H256, (), (), ());

#[derive(Clone, Copy, Debug, Decode, Encode, PartialEq)]
pub struct GenericExtra(Era, Compact<u32>, Compact<u128>);

impl GenericExtra {
    pub fn new(era: Era, nonce: u32) -> GenericExtra {
        GenericExtra(era, Compact(nonce), Compact(0_u128))
    }
}

#[derive(Clone, Copy, Debug, Decode, Encode, PartialEq)]
pub enum Era {
    Immortal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedExtrinsic<Call> {
    /// Address pubkey, Signature, Extras
    pub signature: Option<(GenericAddress, MultiSignature, GenericExtra)>,
    pub function: Call,
}

impl<Call: Encode> UncheckedExtrinsic<Call> {
    pub fn as_hex(&self) -> String {
        format!("0x{}", hex::encode(self.encode()))
    }
}

impl<Call> Encode for UncheckedExtrinsic<Call>
where
    Call: Encode,
{
    fn encode(&self) -> Vec<u8> {
        encode_with_vec_prefix::<Self, _>(|v| {
            match self.signature.as_ref() {
                Some(s) => {
                    v.push(4 | 0b1000_0000);
                    s.encode_to(v);
                }
                None => {
                    v.push(4 & 0b0111_1111);
                }
            }
            self.function.encode_to(v);
        })
    }
}

impl<Call> Decode for UncheckedExtrinsic<Call>
where
    Call: Decode + Encode,
{
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        // This is a little more complicated than usual since the binary format must be
        // compatible with substrate's generic `Vec<u8>` type. Basically this
        // just means accepting that there will be a prefix of vector length (we
        // don't need to use this).
        let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

        let version = input.read_byte()?;

        let is_signed = version & 0b1000_0000 != 0;
        let version = version & 0b0111_1111;
        if version != 4 {
            return Err("Invalid transaction version".into());
        }

        Ok(UncheckedExtrinsic {
            signature: if is_signed {
                Some(Decode::decode(input)?)
            } else {
                None
            },
            function: Decode::decode(input)?,
        })
    }
}

/// Same function as in primitives::generic. Needed to be copied as it is
/// private there.
fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(encoder: F) -> Vec<u8> {
    let size = core::mem::size_of::<T>();
    let reserve = match size {
        0..=0b0011_1111 => 1,
        0b0100_0000..=0b0011_1111_1111_1111 => 2,
        _ => 4,
    };
    let mut v = Vec::with_capacity(reserve + size);
    v.resize(reserve, 0);
    encoder(&mut v);

    // need to prefix with the total length to ensure it's binary compatible with
    // Vec<u8>.
    let mut length: Vec<()> = Vec::new();
    length.resize(v.len() - reserve, ());
    length.using_encoded(|s| {
        v.splice(0..reserve, s.iter().cloned());
    });

    v
}

#[derive(Clone, Copy, Debug, Encode)]
pub struct SignedPayload<Call>((Call, GenericExtra, SignedExtra));

impl<Call: Encode> SignedPayload<Call> {
    pub fn new(call: Call, extra: GenericExtra, s_extra: SignedExtra) -> SignedPayload<Call> {
        SignedPayload((call, extra, s_extra))
    }

    /// Get an encoded version of this payload.
    ///
    /// Payloads longer than 256 bytes are going to be `blake2_256`-hashed.
    pub fn encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        self.0.using_encoded(|payload| {
            if payload.len() > 256 {
                f(&blake2_256(payload))
            } else {
                f(payload)
            }
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
    pub spec_version: u32,
    pub transaction_version: u32,
}

/// Redefinition from `pallet-balances`.
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, Deserialize)]
pub struct AccountDataGen<Balance> {
    /// Non-reserved part of the balance. There may still be restrictions on
    /// this, but it is the total pool what may in principle be transferred,
    /// reserved and used for tipping.
    ///
    /// This is the only balance that matters in terms of most operations on
    /// tokens. It alone is used to determine the balance when in the
    /// contract execution environment.
    pub free: Balance,
    /// Balance which is reserved and may not be used at all.
    ///
    /// This can still get slashed, but gets slashed last of all.
    ///
    /// This balance is a 'reserve' balance that other subsystems use in order
    /// to set aside tokens that are still 'owned' by the account holder,
    /// but which are suspendable.
    pub reserved: Balance,
    /// The amount that `free` may not drop below when withdrawing for *anything
    /// except transaction fee payment*.
    pub misc_frozen: Balance,
    /// The amount that `free` may not drop below when withdrawing specifically
    /// for transaction fee payment.
    pub fee_frozen: Balance,
}

/// Type used to encode the number of references an account has.
pub type RefCount = u32;
/// Index of a transaction.
pub type Index = u32;

/// Redefinition from `frame-system`.
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, Deserialize)]
pub struct AccountInfoGen<Index, AccountData> {
    /// The number of transactions this account has sent.
    pub nonce: Index,
    /// The number of other pallets that currently depend on this account's
    /// existence. The account cannot be reaped until this is zero.
    pub consumers: RefCount,
    /// The number of other pallets that allow this account to exist. The
    /// account may not be reaped until this and `sufficients` are both
    /// zero.
    pub providers: RefCount,
    /// The number of pallets that allow this account to exist for their own
    /// purposes only. The account may not be reaped until this and
    /// `providers` are both zero.
    pub sufficients: RefCount,
    /// The additional data that belongs to this account. Used to store the
    /// balance(s) in a lot of chains.
    pub data: AccountData,
}

pub type AccountData = AccountDataGen<u128>;
pub type AccountInfo = AccountInfoGen<Index, AccountData>;
