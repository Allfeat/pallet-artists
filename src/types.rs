use super::*;
use allfeat_support::types::actors::artist::{ArtistData, CandidateData};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type ArtistOf<T> = ArtistData<
    BoundedVec<u8, <T as Config>::NameMaxLength>,
    <T as frame_system::Config>::BlockNumber,
>;
pub type CandidateOf<T> = CandidateData<
    BoundedVec<u8, <T as Config>::NameMaxLength>,
    <T as frame_system::Config>::BlockNumber,
>;
