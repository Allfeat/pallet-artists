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

    /// Ensure that the caller is an artist sending a signed tx
    /// Same API of `ensure_signed()`
    pub fn ensure_artist(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
        let caller = ensure_signed(origin)?;
        if !Self::is_artist(&caller) {
            return Err(Error::<T>::NotAnArtist)?;
        }
        Ok(caller)
    }
    /// Ensure that the caller is an artist sending a signed tx
    /// Same API of `ensure_signed()`
    pub fn ensure_candidate(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
        let caller = ensure_signed(origin)?;
        if !Self::is_candidate(&caller) {
            return Err(Error::<T>::NotACandidate)?;
        }
        Ok(caller)
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
