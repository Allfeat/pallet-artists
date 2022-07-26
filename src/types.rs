use super::*;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Note: Currently the Artist and the candidate Structure looks similar.
// But there are two different kings of user with different rights
// and their structures could be different as well soon.

/// Structure that holds the artist information that will be stored on-chain
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Artist<AccountId, BoundedString, BlockNumber> {
    /// The identifier of the account of the artist.
    pub(super) account_id: AccountId,
    /// The name of the artist.
    pub(super) name: BoundedString,
    /// The block number when the artist was created
    pub(super) created_at: BlockNumber,
}

/// Structure that holds the candidate information that will be stored on-chain
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Candidate<AccountId, BoundedString, BlockNumber> {
    /// The identifier of the account of the candidate.
    pub(super) account_id: AccountId,
    /// The name of the future artist.
    pub(super) name: BoundedString,
    /// The block number when the candidature was submitted
    pub(super) created_at: BlockNumber,
}

impl<AccountId, BoundedString, BlockNumber> From<Candidate<AccountId, BoundedString, BlockNumber>>
    for Artist<AccountId, BoundedString, BlockNumber>
{
    fn from(candidate: Candidate<AccountId, BoundedString, BlockNumber>) -> Self {
        Artist {
            account_id: candidate.account_id,
            name: candidate.name,
            created_at: candidate.created_at,
        }
    }
}
