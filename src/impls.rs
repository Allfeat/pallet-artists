use super::*;
use frame_support::traits::Contains;

// Expose a public API to check if an `AccountId` is an Artist or a Candidiate from other pallets.
impl<T: Config> Contains<T::AccountId> for Pallet<T> {
    fn contains(t: &T::AccountId) -> bool {
        <Artists<T>>::contains_key(t) || <Candidates<T>>::contains_key(t)
    }
}
