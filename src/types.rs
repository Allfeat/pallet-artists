use super::*;
use allfeat_support::types::actors::artist::{ArtistData, CandidateData};
use frame_system::pallet_prelude::BlockNumberFor;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type ArtistOf<T> = ArtistData<BoundedVec<u8, <T as Config>::NameMaxLength>, BlockNumberFor<T>>;
pub type CandidateOf<T> =
    CandidateData<BoundedVec<u8, <T as Config>::NameMaxLength>, BlockNumberFor<T>>;
