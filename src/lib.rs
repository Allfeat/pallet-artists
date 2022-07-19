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

// TODO: Link with pallet_collective or pallet_membership
// TODO: Link with pallet_identity (eg: MinRankOfClass from ranked-collective)

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + TypeInfo {
        /// Let the pallet to emit events
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Used for candidate/artist deposit
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Who can certificate an Artist
        type ArtistsManagerOrigin: EnsureOrigin<Self::Origin>;

        /// The receiver of the signal for when the membership has been initialized.
        /// This happens pre-genesis and will usually be the same as `MembershipChanged`.
        /// If you need to do something different on initialization, then you can change
        /// this accordingly.
        type MembershipInitialized: InitializeMembers<Self::AccountId>;

        /// The receiver of the signal for when the members have changed.
        type MembershipChanged: ChangeMembers<Self::AccountId>;

        /// The deposit needed for creating an artist account.
        #[pallet::constant]
        type CreationDepositAmount: Get<BalanceOf<Self>>;

        /// The maximum number of artists that can be stored.
        #[pallet::constant]
        type MaxArtists: Get<u32>;

        /// The maximum number of candidates that can be stored.
        #[pallet::constant]
        type MaxCandidates: Get<u32>;

        /// The maximum length of an artist name or symbol stored on-chain.
        #[pallet::constant]
        type NameMaxLength: Get<u32>;
    }

    #[pallet::storage]
    #[pallet::getter(fn get_candidate)]
    pub(super) type Candidates<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Candidate<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_artist)]
    pub(super) type Artists<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Artist<T>, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// The existing artists at the genesis
        pub artists: Vec<(T::AccountId, Vec<u8>)>,
        /// The existing artists at the genesis
        pub candidates: Vec<(T::AccountId, Vec<u8>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                artists: Default::default(),
                candidates: Default::default(),
            }
        }
    }

    // TODO: Duplicate code between artist and candidate loops
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (account_id, name) in &self.artists {
                let who: T::AccountId = account_id
                    .clone()
                    .try_into()
                    .expect("Error while getting the artist account id");
                let name: BoundedVec<u8, T::NameMaxLength> = name
                    .clone()
                    .try_into()
                    .expect("Error while formatting the artist name");

                if Artists::<T>::contains_key(&account_id) {
                    panic!("Artist already added to the list")
                }

                T::Currency::reserve(&account_id, T::CreationDepositAmount::get())
                    .expect("Could not reverse deposit for the candidate");

                let artist = Artist {
                    account_id: who,
                    name,
                    created_at: <frame_system::Pallet<T>>::block_number(),
                };

                Artists::<T>::insert(&account_id, artist);
            }

            for (account_id, name) in &self.candidates {
                let who: T::AccountId = account_id
                    .clone()
                    .try_into()
                    .expect("Error while getting the candidate account id");
                let name: BoundedVec<u8, T::NameMaxLength> = name
                    .clone()
                    .try_into()
                    .expect("Error while formatting the candidate name");

                if Candidates::<T>::contains_key(&account_id) {
                    panic!("Candidate already added to the list")
                }

                T::Currency::reserve(&account_id, T::CreationDepositAmount::get())
                    .expect("Could not reverse deposit for the candidate");

                let candidate = Candidate {
                    account_id: who,
                    name,
                    created_at: <frame_system::Pallet<T>>::block_number(),
                };

                Candidates::<T>::insert(&account_id, candidate);
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Candidate events:
        // =================
        /// An entity has issued a candidacy. See the transaction for who.
        CandidateAdded(T::AccountId),
        /// An entity withdrew candidacy. See the transaction for who.
        CandidateWithdrew(T::AccountId),
        /// An artist was created from a candidate after approbation.
        /// This artist is also added to the artist membership
        CandidateApproved(T::AccountId),

        // Artist events:
        // ==============
        /// An artist has been updated
        ArtistUpdated(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        // General errors:
        // ===============
        /// The caller doesn't have enough funds for the deposit
        NotEnoughFunds,
        /// The given string is longer than `T::NameMaxLength`.
        NameTooLong,

        // Candidate related errors:
        // =========================
        /// The account is already in the candidate list
        AlreadyACandidate,
        /// The number of artists stored exceeded `T::MaxCandidates`.
        ExceedCandidateBound,
        /// The wanted candidate is not found in the Candidates Storage
        CandidateNotFound,
        /// The caller isn't in the candidate list.
        NotACandidate,

        // Artist related errors:
        // ======================
        /// This account already is a certificated artist account.
        AlreadyAnArtist,
        /// The number of artists stored exceeded `T::MaxArtists`.
        ExceedArtistBound,
        /// The caller isn't a verified artist.
        NotAnArtist,
        /// The wanted artist is not found in the Artists Storage
        ArtistNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// To be an artist, the caller have to candidate first.
        /// This will create the candidate profile with the given fields:
        ///
        /// `name:` The name of the artist.
        ///
        /// NOTE: This can only be done once for an account.
        #[pallet::weight(0)]
        pub fn submit_candidacy(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Check if the caller isn't neither a candidate nor an artist
            ensure!(!Self::is_artist(&caller), Error::<T>::AlreadyAnArtist);
            ensure!(!Self::is_candidate(&caller), Error::<T>::AlreadyACandidate);

            // Check and format candidate attributes
            let name: BoundedVec<u8, T::NameMaxLength> =
                name.try_into().map_err(|_| Error::<T>::NameTooLong)?;
            let created_at: T::BlockNumber = <frame_system::Pallet<T>>::block_number();

            Self::reserve_deposit(&caller)?;

            // Create the candidate and store it on-chain
            <Candidates<T>>::insert(
                caller.clone(),
                Candidate {
                    account_id: caller.clone(),
                    name,
                    created_at,
                },
            );

            Self::deposit_event(Event::<T>::CandidateAdded(caller));

            Ok(())
        }

        /// Withdraw candidacy to become an artist and get deposit back.
        #[pallet::weight(0)]
        pub fn withdraw_candidacy(origin: OriginFor<T>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // TODO: Ensure origin is in the candidate collective instead
            if !Self::is_candidate(&caller) {
                return Err(Error::<T>::NotACandidate)?;
            }

            <Candidates<T>>::remove(&caller);

            // returns deposit to the caller
            Self::unreserve_deposit(&caller)?;

            Self::deposit_event(Event::<T>::CandidateWithdrew(caller));

            Ok(())
        }

        /// Approve a candidate and level up his account the an artist.
        /// For simplicity, this function use membership managed rights.
        /// Later, a more complex validation logic could be implemented.
        ///
        /// May only be called from `T::ApproveOrigin`.
        ///
        /// TODO: impl kyc verification
        #[pallet::weight(0)]
        pub fn approve_candidacy(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            // TODO: Use collective based origin
            ensure_root(origin)?;
            // T::ArtistsManagerOrigin::ensure_origin(origin.clone())?;

            if Self::is_artist(&who) {
                return Err(Error::<T>::AlreadyAnArtist)?;
            }

            // Create an Artist from the candidate and store it on-chain
            let artist: Artist<T> = <Candidates<T>>::try_get(&who)
                .or_else(|_| Err(Error::<T>::CandidateNotFound))?
                .into();

            <Artists<T>>::insert(who.clone(), artist);

            // Then remove the candidature
            <Candidates<T>>::remove(&who);

            Self::deposit_event(Event::<T>::CandidateApproved(who));
            Ok(())
        }
    }
}
