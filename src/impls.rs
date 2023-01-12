use super::*;
use allfeat_support::traits::actors::{artist::ArtistStorage, ActorStorage};

impl<T: Config> ActorStorage<T::AccountId> for Pallet<T> {
    /// THIS SHOULDN'T BE USED
    /// We don't want to use this function as the artist module is divided in two actors
    fn is_actor(_: &T::AccountId) -> bool {
        return false;
    }
}

impl<T: Config> ArtistStorage<T::AccountId, CandidateOf<T>, ArtistOf<T>> for Pallet<T> {
    fn is_candidate(account_id: &T::AccountId) -> bool {
        <Candidates<T>>::contains_key(account_id)
    }
    fn is_artist(account_id: &T::AccountId) -> bool {
        <Artists<T>>::contains_key(account_id)
    }
    fn candidate(account_id: &T::AccountId) -> Option<CandidateOf<T>> {
        <Candidates<T>>::get(account_id)
    }
    fn artist(account_id: &T::AccountId) -> Option<ArtistOf<T>> {
        <Artists<T>>::get(account_id)
    }
}
