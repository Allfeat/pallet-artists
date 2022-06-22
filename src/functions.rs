use super::*;
use frame_support::traits::Get;

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    // Assets relative
    pub fn create_and_init_asset(
        id: T::AssetId,
        account: T::AccountId,
        name: Vec<u8>,
        symbol: Vec<u8>,
    ) -> Result<(), DispatchError> {
        // Create the asset of the new artist
        T::Assets::create(id, account.clone(), false, T::MinBalance::get())?;
        // Set the metadatas of the artist asset
        T::Assets::set(id, &account, name, symbol, T::Decimals::get())?;
        // Mint the default supply of the artist asset
        T::Assets::mint_into(id, &account, T::DefaultSupply::get())?;

        Ok(())
    }

    /// Add a new artist account to the upstream `T::ArtistGroup` and the `Members` pallet storage.
    pub fn add_artist_account(acc: T::AccountId) -> Result<(), DispatchError> {
        let mut accounts: Vec<T::AccountId> = Members::<T, I>::get().into();

        // find the correct index and check if the element doesn't already exist
        let location = accounts
            .binary_search(&acc)
            .err()
            .ok_or(Error::<T, I>::AlreadyUsedAcc)?;
        accounts.insert(location, acc.clone());

        T::ArtistGroup::change_members_sorted(&[acc], &[], &accounts[..]);
        let bounded_accounts: BoundedVec<T::AccountId, T::MaxArtists> = accounts
            .try_into()
            .map_err(|_| Error::<T, I>::ExceedArtistBound)?;
        Members::<T, I>::put(bounded_accounts);

        Ok(())
    }
}
