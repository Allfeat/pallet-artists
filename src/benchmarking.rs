//! Artists pallet benchmarking.
#![allow(unused_imports)]
#![allow(dead_code)]
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{
    account, benchmarks_instance_pallet, whitelist_account, whitelisted_caller,
};
use frame_support::traits::{EnsureOrigin, Get};
use frame_system::RawOrigin as SystemOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

use crate::Pallet as Artists;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

fn assert_last_event<T: Config, I: 'static>(generic_event: <T as Config>::Event) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn assert_event<T: Config, I: 'static>(generic_event: <T as Config>::Event) {
    frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}
