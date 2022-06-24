// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
pub mod tests;

mod functions;
mod types;
pub use types::*;

use frame_support::{
    dispatch::DispatchError,
    dispatch::DispatchResult,
    traits::{ChangeMembers, Currency, InitializeMembers, ReservableCurrency},
    Blake2_128Concat, BoundedVec,
};
use sp_std::prelude::*;

pub use pallet::*;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Pallet configuration
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: ReservableCurrency<Self::AccountId>;

        type ArtistGroup: ChangeMembers<Self::AccountId> + InitializeMembers<Self::AccountId>;

        /// The deposit needed for creating an artist account.
        #[pallet::constant]
        type CreationDepositAmount: Get<BalanceOf<Self>>;

        /// The maximum number of artists that can be stored.
        #[pallet::constant]
        type MaxArtists: Get<u32>;

        /// The maximum length of an artist name or symbol stored on-chain.
        #[pallet::constant]
        type StringLimit: Get<u32>;

        // type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    pub(super) type CertifiedMembers<T: Config> =
        StorageValue<_, BoundedVec<T::AccountId, ConstU32<1_000_000>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_artist)]
    pub(super) type ArtistStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        ArtistInfos<T::AccountId, BoundedVec<u8, T::StringLimit>, T::BlockNumber>,
        OptionQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// The existing artists at the genesis
        pub artists: Vec<(T::AccountId, bool, Vec<u8>, Vec<Styles>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                artists: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (account, is_certified, name, styles) in &self.artists {
                let name: BoundedVec<u8, T::StringLimit> = name.clone().try_into().unwrap();
                let styles: BoundedVec<Styles, ConstU32<3>> = styles.clone().try_into().unwrap();

                if ArtistStorage::<T>::contains_key(account) {
                    panic!()
                }

                T::Currency::reserve(account, T::CreationDepositAmount::get()).unwrap();

                ArtistStorage::<T>::insert(
                    account,
                    ArtistInfos {
                        account: account.clone(),
                        is_certified: *is_certified,
                        name,
                        styles,
                        age: <frame_system::Pallet<T>>::block_number(),
                    },
                );

                // TODO add certified to members/group
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An artist was created.
        ArtistCreated {
            account: T::AccountId,
            name: BoundedVec<u8, T::StringLimit>,
            block: T::BlockNumber,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This account already is an artist account.
        AlreadyCreated,
        /// The given string is longer than `T::StringLimit`.
        StringTooLong,
        /// The account is already marked as certified account.
        AlreadyCertified,
        /// The number of artists stored exceeded `T::MaxArtists`.
        ExceedArtistBound,
        /// The caller isn't a verified artist.
        NotAnArtist,
        /// The caller doesn't have enough funds for the deposit
        NotEnoughFunds,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Claim the account of the caller origin as an Artist.
        /// This will create the artist profile of the account with the given fields:
        ///
        /// `name:` The name of the artist.
        /// `styles:` The styles associated to the artist, up to 3.
        ///
        /// NOTE: This can only be done once for an account.
        #[pallet::weight(0)]
        pub fn create(
            origin: OriginFor<T>,
            name: Vec<u8>,
            styles: BoundedVec<Styles, ConstU32<3>>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            ensure!(
                !ArtistStorage::<T>::contains_key(&caller),
                Error::<T>::AlreadyCreated
            );

            let bounded_name: BoundedVec<u8, T::StringLimit> =
                name.try_into().map_err(|_| Error::<T>::StringTooLong)?;
            let age: T::BlockNumber = <frame_system::Pallet<T>>::block_number();

            T::Currency::reserve(&caller, T::CreationDepositAmount::get())
                .map_err(|_| Error::<T>::NotEnoughFunds)?;

            ArtistStorage::<T>::insert(
                caller.clone(),
                ArtistInfos {
                    account: caller.clone(),
                    is_certified: false,
                    name: bounded_name.clone(),
                    styles,
                    age: age.clone(),
                },
            );

            Self::deposit_event(Event::<T>::ArtistCreated {
                account: caller,
                name: bounded_name,
                block: age,
            });

            Ok(())
        }
    }
}
