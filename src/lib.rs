// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
pub mod tests;

mod functions;
mod impls;
mod types;
mod weights;

pub use types::*;

use core::marker::PhantomData;
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_support::traits::EnsureOrigin;
use frame_support::weights::Weight;
use frame_support::{
    codec::{Decode, Encode, MaxEncodedLen},
    dispatch::DispatchError,
    dispatch::DispatchResult,
    traits::{Currency, ReservableCurrency},
    Blake2_128Concat, BoundedVec,
};
use scale_info::TypeInfo;
use sp_runtime::traits::Hash;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

pub use pallet::*;

/// Origin for the collective module.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[codec(mel_bound(AccountId: MaxEncodedLen))]
pub enum RawOrigin<AccountId> {
    /// It has been condoned by a single artist.
    Artist(AccountId),
    /// It has been condoned by a single Candidate.
    Candidate(AccountId),
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::weights::WeightInfo;
    use allfeat_support::types::actors::artist::{ArtistData, CandidateData};
    use frame_support::pallet_prelude::*;
    use frame_support::weights::{GetDispatchInfo, PostDispatchInfo};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Dispatchable;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Let the pallet to emit events
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Used for candidate/artist deposit
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The outer origin type.
        type Origin: From<RawOrigin<Self::AccountId>>;

        /// Who can certificate an Artist
        type AdminOrigin: EnsureOrigin<<Self as frame_system::Config>::Origin>;

        type Call: Parameter
            + Dispatchable<Origin = <Self as Config>::Origin, PostInfo = PostDispatchInfo>
            + From<frame_system::Call<Self>>
            + GetDispatchInfo;

        /// The deposit needed for creating an artist account.
        #[pallet::constant]
        type CreationDepositAmount: Get<BalanceOf<Self>>;

        /// The maximum length of an artist name or symbol stored on-chain.
        #[pallet::constant]
        type NameMaxLength: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::origin]
    pub type Origin<T> = RawOrigin<<T as frame_system::Config>::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn get_candidate)]
    pub(super) type Candidates<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, CandidateOf<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_artist)]
    pub(super) type Artists<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, ArtistOf<T>, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// The existing artists at the genesis
        pub artists: Vec<(T::AccountId, Vec<u8>)>,
        /// The existing candidates at the genesis
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

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (account_id, name) in &self.artists {
                let name: BoundedVec<u8, T::NameMaxLength> = name
                    .clone()
                    .try_into()
                    .expect("Error while formatting the artist name");

                if Artists::<T>::contains_key(&account_id) {
                    panic!("Artist already added to the list")
                }

                T::Currency::reserve(&account_id, T::CreationDepositAmount::get())
                    .expect("Could not reverse deposit for the candidate");

                let artist = ArtistData {
                    name,
                    created_at: <frame_system::Pallet<T>>::block_number(),
                };

                Artists::<T>::insert(&account_id, artist);
            }

            for (account_id, name) in &self.candidates {
                let name: BoundedVec<u8, T::NameMaxLength> = name
                    .clone()
                    .try_into()
                    .expect("Error while formatting the candidate name");

                if Candidates::<T>::contains_key(&account_id) {
                    panic!("Candidate already added to the list")
                }

                T::Currency::reserve(&account_id, T::CreationDepositAmount::get())
                    .expect("Could not reverse deposit for the candidate");

                let candidate = CandidateData {
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
        /// A Candidate called an extrinsic
        CandidateExecuted {
            dispatch_hash: T::Hash,
            result: DispatchResult,
        },

        // Artist events:
        // ==============
        /// An Artist called an extrinsic
        ArtistExecuted {
            dispatch_hash: T::Hash,
            result: DispatchResult,
        },
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
        /// The wanted candidate is not found in the Candidates Storage
        CandidateNotFound,
        /// The caller isn't in the candidate list.
        NotACandidate,

        // Artist related errors:
        // ======================
        /// This account already is a certificated artist account.
        AlreadyAnArtist,
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
        #[pallet::weight(T::WeightInfo::submit_candidacy(T::NameMaxLength::get()))]
        pub fn submit_candidacy(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Check if the caller isn't neither a candidate nor an artist
            ensure!(!Self::is_artist(&caller), Error::<T>::AlreadyAnArtist);
            ensure!(!Self::is_candidate(&caller), Error::<T>::AlreadyACandidate);

            let candidate = CandidateData {
                name: name.try_into().map_err(|_| Error::<T>::NameTooLong)?,
                created_at: <frame_system::Pallet<T>>::block_number(),
            };

            Self::reserve_deposit(&caller)?;

            <Candidates<T>>::insert(caller.clone(), candidate);

            Self::deposit_event(Event::<T>::CandidateAdded(caller));

            Ok(())
        }

        /// Withdraw candidacy to become an artist and get deposit back.
        #[pallet::weight(T::WeightInfo::withdraw_candidacy())]
        pub fn withdraw_candidacy(origin: OriginFor<T>) -> DispatchResult {
            let caller = Self::ensure_candidate(origin)?;

            <Candidates<T>>::remove(&caller);

            // returns deposit to the caller
            Self::unreserve_deposit(&caller)?;

            Self::deposit_event(Event::<T>::CandidateWithdrew(caller));

            Ok(())
        }

        /// Approve a candidate and level up his account to be an artist.
        ///
        /// May only be called from `T::AdminOrigin`.
        #[pallet::weight(T::WeightInfo::approve_candidacy(T::NameMaxLength::get()))]
        pub fn approve_candidacy(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?;

            ensure!(!Self::is_artist(&who), Error::<T>::AlreadyAnArtist);

            let candidate =
                <Candidates<T>>::try_get(&who).or_else(|_| Err(Error::<T>::CandidateNotFound))?;

            let artist = ArtistData {
                name: candidate.name,
                created_at: <frame_system::Pallet<T>>::block_number(),
            };

            <Artists<T>>::insert(who.clone(), artist);

            <Candidates<T>>::remove(&who);

            Self::deposit_event(Event::<T>::CandidateApproved(who));
            Ok(())
        }

        #[pallet::weight(
            T::WeightInfo::call_as_artist()
                .saturating_add(call.get_dispatch_info().weight)
        )]
        pub fn call_as_artist(
            origin: OriginFor<T>,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(Self::is_artist(&caller), Error::<T>::NotAnArtist);

            let dispatch_hash = T::Hashing::hash_of(&call);
            let result = call.dispatch(RawOrigin::Artist(caller).into());

            Self::deposit_event(Event::<T>::ArtistExecuted {
                dispatch_hash,
                result: result.map(|_| ()).map_err(|e| e.error),
            });

            Ok(get_result_weight(result)
                .map(|w| T::WeightInfo::call_as_artist().saturating_add(w))
                .into())
        }

        #[pallet::weight(
            T::WeightInfo::call_as_candidate()
                .saturating_add(call.get_dispatch_info().weight)
        )]
        pub fn call_as_candidate(
            origin: OriginFor<T>,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(Self::is_candidate(&caller), Error::<T>::NotACandidate);

            let dispatch_hash = T::Hashing::hash_of(&call);
            let result = call.dispatch(RawOrigin::Candidate(caller).into());

            Self::deposit_event(Event::<T>::CandidateExecuted {
                dispatch_hash,
                result: result.map(|_| ()).map_err(|e| e.error),
            });

            Ok(get_result_weight(result)
                .map(|w| T::WeightInfo::call_as_candidate().saturating_add(w))
                .into())
        }
    }
}

/// Return the weight of a dispatch call result as an `Option`.
///
/// Will return the weight regardless of what the state of the result is.
fn get_result_weight(result: DispatchResultWithPostInfo) -> Option<Weight> {
    match result {
        Ok(post_info) => post_info.actual_weight,
        Err(err) => err.post_info.actual_weight,
    }
}

pub struct EnsureArtist<AccountId>(PhantomData<AccountId>);
impl<O: Into<Result<RawOrigin<AccountId>, O>> + From<RawOrigin<AccountId>>, AccountId: Decode>
    EnsureOrigin<O> for EnsureArtist<AccountId>
{
    type Success = AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        o.into().and_then(|o| match o {
            RawOrigin::Artist(id) => Ok(id),
            _ => Err(O::from(o)),
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        let zero_account_id =
            AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed");
        Ok(O::from(RawOrigin::Artist(zero_account_id)))
    }
}

pub struct EnsureCandidate<AccountId>(PhantomData<AccountId>);
impl<O: Into<Result<RawOrigin<AccountId>, O>> + From<RawOrigin<AccountId>>, AccountId: Decode>
    EnsureOrigin<O> for EnsureCandidate<AccountId>
{
    type Success = AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        o.into().and_then(|o| match o {
            RawOrigin::Candidate(id) => Ok(id),
            _ => Err(O::from(o)),
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        let zero_account_id =
            AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
                .expect("infinite length input; no invalid inputs for type; qed");
        Ok(O::from(RawOrigin::Candidate(zero_account_id)))
    }
}
