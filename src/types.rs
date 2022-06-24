use super::*;
use frame_support::pallet_prelude::*;

use serde::{Serialize, Deserialize};

/// The main informations stored on-chain for an artist.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct ArtistInfos<AccountId, BoundedString, BlockNumber> {
    /// The identifier of the account of the artist.
    pub(super) account: AccountId,
    /// The artist is certified or not.
    pub(super) is_certified: bool,
    /// The name of the artist.
    pub(super) name: BoundedString,
    /// The musical styles of the artist, up to 3.
    pub(super) styles: BoundedVec<Styles, ConstU32<3>>,
    /// The block number when the artist was created
    pub(super) age: BlockNumber,
}

/// A list of music styles an artist profile can include.
#[derive(
    RuntimeDebug,
    Clone,
    Encode,
    Decode,
    Eq,
    PartialEq,
    MaxEncodedLen,
    TypeInfo,
    Serialize,
    Deserialize,
)]
pub enum Styles {
    Electronic,
    Pop,
    Rock,
    Blues,
    Country,
    Folk,
    HipHop,
    Jazz,
    Metal,
}
