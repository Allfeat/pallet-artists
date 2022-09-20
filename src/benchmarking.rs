//! Artists pallet benchmarking.
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::Get;
use frame_system::RawOrigin as SystemOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

/// Helper function that generates a random string from a given length
fn generate_string(length: usize) -> Vec<u8> {
    vec![1; length]
}

fn create_candidacy<T: Config>(caller: T::AccountId, name: Vec<u8>) -> DispatchResult {
    Pallet::<T>::submit_candidacy(SystemOrigin::Signed(caller).into(), name)
}

fn approve_candidacy_of<T: Config>(caller: T::AccountId) -> DispatchResult {
    Pallet::<T>::approve_candidacy(SystemOrigin::Root.into(), caller)
}

benchmarks! {
    where_clause { where T: Config }

    submit_candidacy {
        let n in 1..T::NameMaxLength::get();
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
    }: _(SystemOrigin::Signed(caller.clone()), generate_string(n.try_into().unwrap()))
    verify {
        assert_last_event::<T>(Event::CandidateAdded { 0: caller }.into());
    }

    withdraw_candidacy {
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        create_candidacy::<T>(caller.clone(), generate_string(T::NameMaxLength::get() as usize))?;
    }: _(SystemOrigin::Signed(caller.clone()))
    verify {
        assert_last_event::<T>(Event::CandidateWithdrew { 0: caller }.into());
    }

    approve_candidacy {
        let n in 1..T::NameMaxLength::get();
        let admin: T::AccountId = whitelisted_caller();
        let candidate: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&candidate, BalanceOf::<T>::max_value());
        create_candidacy::<T>(candidate.clone(), generate_string(n.try_into().unwrap()))?;
    }: _(SystemOrigin::Root, candidate.clone())
    verify {
        assert_last_event::<T>(Event::CandidateApproved { 0: candidate }.into());
    }

    call_as_artist {
        let artist: T::AccountId = whitelisted_caller();
        let call: <T as Config>::Call = frame_system::Call::<T>::remark { remark: vec![] }.into();
        T::Currency::make_free_balance_be(&artist, BalanceOf::<T>::max_value());
        create_candidacy::<T>(artist.clone(), generate_string(T::NameMaxLength::get() as usize))?;
        approve_candidacy_of::<T>(artist.clone())?;
    }: _(SystemOrigin::Signed(artist.clone()), Box::new(call.clone()))
    verify {
        let dispatch_hash = T::Hashing::hash_of(&call);
        // Note that execution fails due to mis-matched origin
        assert_last_event::<T>(
            Event::ArtistExecuted { dispatch_hash, result: Err(DispatchError::BadOrigin) }.into()
        );
    }

    call_as_candidate {
        let candidate: T::AccountId = account("alice", 0, 0);
        let call: <T as Config>::Call = frame_system::Call::<T>::remark { remark: vec![] }.into();
        T::Currency::make_free_balance_be(&candidate, BalanceOf::<T>::max_value());
        create_candidacy::<T>(candidate.clone(), generate_string(T::NameMaxLength::get() as usize))?;
    }: _(SystemOrigin::Signed(candidate.clone()), Box::new(call.clone()))
    verify {
        let dispatch_hash = T::Hashing::hash_of(&call);
        // Note that execution fails due to mis-matched origin
        assert_last_event::<T>(
            Event::CandidateExecuted { dispatch_hash, result: Err(DispatchError::BadOrigin) }.into()
        );
    }
}

impl_benchmark_test_suite! {
    Pallet,
    crate::mock::new_test_ext(false),
    crate::mock::Test
}
