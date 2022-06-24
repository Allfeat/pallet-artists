use super::*;
use frame_support::pallet_prelude::ConstU32;

impl<T: Config> Pallet<T> {
    /// Add a new certified artist account to the upstream `T::ArtistGroup` and the `CertifiedMembers` pallet storage.
    pub fn add_certif_artist_account(acc: T::AccountId) -> Result<(), DispatchError> {
        let mut accounts: Vec<T::AccountId> = CertifiedMembers::<T>::get().into();

        // find the correct index and check if the element doesn't already exist
        let location = accounts
            .binary_search(&acc)
            .err()
            .ok_or(Error::<T>::AlreadyCertified)?;
        accounts.insert(location, acc.clone());

        T::ArtistGroup::change_members_sorted(&[acc], &[], &accounts[..]);
        let bounded_accounts: BoundedVec<T::AccountId, ConstU32<1_000_000>> =
            accounts
                .try_into()
                .map_err(|_| Error::<T>::ExceedArtistBound)?;
        CertifiedMembers::<T>::put(bounded_accounts);

        Ok(())
    }
}
