use std::str::FromStr;

use parity_scale_codec::{Compact, Decode, Encode, Error, Input};
use serde::{Deserialize, Serialize};
pub use sp_core::crypto::{AccountId32, Ss58AddressFormat, Ss58AddressFormatRegistry, Ss58Codec};
pub use sp_core::ecdsa::{Public, Signature};
pub use sp_core::{blake2_256, H256};

use crate::pallets::timestamp::decode_timestamp;
use crate::utils::deser_number_or_hex;

pub mod client;
pub mod network;
pub mod pallets;
pub mod rpc;
mod utils;

pub type GenericAddress = MultiAddress<AccountId32, ()>;

pub type Balance = u128;

#[derive(Clone, Debug, Decode, Encode, PartialEq, Eq)]
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

impl From<AccountId32> for GenericAddress {
    fn from(a: AccountId32) -> Self {
        MultiAddress::Id(a)
    }
}

impl From<Public> for GenericAddress {
    fn from(p: Public) -> Self {
        let acct = public_into_account(p);
        MultiAddress::Id(acct)
    }
}

impl FromStr for GenericAddress {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccountId32::from_ss58check(s)
            .map_err(|_| "Expected ss58 address")?
            .into())
    }
}

pub fn public_into_account(p: Public) -> AccountId32 {
    let hash = blake2_256(&p.0);
    hash.into()
}

#[derive(Clone, Debug, Decode, Encode, PartialEq, Eq)]
pub enum MultiSignature {
    /// An ECDSA/SECP256k1 signature.
    #[codec(index = 2)]
    Ecdsa(Signature),
}

/// runtime spec verion, transaction version, genesis hash, genesis hash or
/// current hash, ...others
pub type SignedExtra = (u32, u32, H256, H256, (), (), ());

#[derive(Clone, Copy, Debug, Decode, Encode, PartialEq, Eq)]
pub struct GenericExtra(Era, Compact<u32>, Compact<Balance>);

impl GenericExtra {
    pub fn new(era: Era, nonce: u32) -> GenericExtra {
        GenericExtra(era, Compact(nonce), Compact(0u128))
    }
}

#[derive(Clone, Copy, Debug, Decode, Encode, PartialEq, Eq)]
pub enum Era {
    Immortal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UncheckedExtrinsic<Call> {
    /// Address pubkey, Signature, Extras
    pub signature: Option<(GenericAddress, MultiSignature, GenericExtra)>,
    pub function: Call,
}

impl<Call: Encode> UncheckedExtrinsic<Call> {
    pub fn as_hex(&self) -> String {
        format!("0x{}", hex::encode(self.encode()))
    }

    pub fn call_as_hex(&self) -> String {
        format!("0x{}", hex::encode(self.function.encode()))
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

pub type AccountData = AccountDataGen<Balance>;
pub type AccountInfo = AccountInfoGen<Index, AccountData>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeDetails {
    /// The minimum fee for a transaction to be included in a block.
    pub inclusion_fee: Option<InclusionFee>,
    // Do not serialize and deserialize `tip` as we actually can not pass any tip to the RPC.
    #[serde(skip)]
    #[serde(deserialize_with = "deser_number_or_hex")]
    pub tip: Balance,
}

impl FeeDetails {
    /// Returns the final fee.
    ///
    /// final_fee = inclusion_fee + tip;
    pub fn final_fee(&self) -> Balance {
        self.inclusion_fee
            .as_ref()
            .map(|i| i.inclusion_fee())
            .unwrap_or_default()
            .saturating_add(self.tip)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InclusionFee {
    /// This is the minimum amount a user pays for a transaction. It is declared
    /// as a base _weight_ in the runtime and converted to a fee using
    /// `WeightToFee`.
    #[serde(deserialize_with = "deser_number_or_hex")]
    pub base_fee: Balance,
    /// The length fee, the amount paid for the encoded length (in bytes) of the
    /// transaction.
    #[serde(deserialize_with = "deser_number_or_hex")]
    pub len_fee: Balance,
    /// - `targeted_fee_adjustment`: This is a multiplier that can tune the
    ///   final fee based on the congestion of the network.
    /// - `weight_fee`: This amount is computed based on the weight of the
    ///   transaction. Weight
    /// accounts for the execution time of a transaction.
    ///
    /// adjusted_weight_fee = targeted_fee_adjustment * weight_fee
    #[serde(deserialize_with = "deser_number_or_hex")]
    pub adjusted_weight_fee: Balance,
}

impl InclusionFee {
    pub fn inclusion_fee(&self) -> Balance {
        self.base_fee
            .saturating_add(self.len_fee)
            .saturating_add(self.adjusted_weight_fee)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedBlock {
    pub block: Block,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub header: Header,
    pub extrinsics: Vec<String>,
}

impl Block {
    /// Get timestamp that is set on the Block
    pub fn timestamp(&self) -> Option<u64> {
        self.extrinsics.iter().find_map(|e| decode_timestamp(e))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub parent_hash: H256,
    #[serde(deserialize_with = "deser_number_or_hex")]
    pub number: u128,
}
