//! Artists pallet benchmarking.
#![allow(unused_imports)]
#![allow(dead_code)]
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{
    account, benchmarks, impl_benchmark_test, impl_benchmark_test_suite, whitelist_account,
    whitelisted_caller,
};
use frame_support::traits::{EnsureOrigin, Get};
use frame_system::RawOrigin as SystemOrigin;
use rand::{thread_rng, Rng};
use sp_runtime::traits::Bounded;
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

use crate::Pallet as Artists;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

/// Helper function that generates a random string from a given length
fn generate_random_string(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::with_capacity(length);
    let mut rng = thread_rng();
    for _ in 0..length {
        let x: usize = rng.gen();
        result.push(chars[x % chars.len()])
    }
    result
}

benchmarks! {
    where_clause { where T: Config }

    submit_candidacy {
        let n in 1..T::NameMaxLength::get();
        let caller: T::AccountId = whitelisted_caller();
        let caller_lookup = T::Lookup::unlookup(caller.clone());
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
    }: _(SystemOrigin::Signed(caller.clone()), generate_random_string(n.try_into().unwrap()).as_bytes().to_vec())
}

impl_benchmark_test_suite! {
    Pallet,
    crate::mock::new_test_ext(false),
    crate::mock::Test
}
