use super::*;
use frame_support::traits::Get;
use frame_system::{ensure_signed, pallet_prelude::OriginFor};

impl<T: Config> Pallet<T> {
    /// Check if the given account_id is an artist
    pub fn is_artist(account_id: &T::AccountId) -> bool {
        <Artists<T>>::contains_key(account_id)
    }

    /// Check if the given account_id is a candidate
    pub fn is_candidate(account_id: &T::AccountId) -> bool {
        <Candidates<T>>::contains_key(account_id)
    }

    /// Function with the same API of `ensure_signed()`
    /// that check if the origin is an artist (via membership)
    // TODO: Replace this by T::Collective_ABC::ensure_origin()
    pub fn ensure_artist(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
        let who = ensure_signed(origin)?;
        if Self::is_artist(&who) {
            return Err(Error::<T>::NotAnArtist)?;
        }
        Ok(who)
    }

    pub fn reserve_deposit(caller: &T::AccountId) -> DispatchResult {
        let deposit = T::CreationDepositAmount::get();
        T::Currency::reserve(caller, deposit).map_err(|_| Error::<T>::NotEnoughFunds)?;
        Ok(())
    }

    pub fn unreserve_deposit(to: &T::AccountId) -> DispatchResult {
        let deposit = T::CreationDepositAmount::get();
        T::Currency::unreserve(to, deposit);
        Ok(())
    }
}
