
//! Autogenerated weights for `pallet_artists`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-20, STEPS: `50`, REPEAT: 30, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `DESKTOP-11NSJM8`, CPU: `Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/allfeat
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_artists
// --extrinsic
// *
// --steps
// 50
// --repeat
// 30
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use frame_support::weights::constants::RocksDbWeight;
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_artists.
pub trait WeightInfo {
	fn submit_candidacy(_n: u32, ) -> Weight;
	fn withdraw_candidacy() -> Weight;
	fn approve_candidacy(n: u32, ) -> Weight;
	fn call_as_artist() -> Weight;
	fn call_as_candidate() -> Weight;
}

impl WeightInfo for () {
	// Storage: Artists Artists (r:1 w:0)
	// Storage: Artists Candidates (r:1 w:1)
	/// The range of component `n` is `[1, 128]`.
	fn submit_candidacy(_n: u32, ) -> Weight {
		Weight::default()
	}
	// Storage: Artists Candidates (r:1 w:1)
	fn withdraw_candidacy() -> Weight {
		Weight::default()

	}
	// Storage: Artists Artists (r:1 w:1)
	// Storage: Artists Candidates (r:1 w:1)
	/// The range of component `n` is `[1, 128]`.
	fn approve_candidacy(_n: u32, ) -> Weight {
		Weight::default()

	}
	// Storage: Artists Artists (r:1 w:0)
	fn call_as_artist() -> Weight {
		Weight::default()
	}
	// Storage: Artists Candidates (r:1 w:0)
	fn call_as_candidate() -> Weight {
		Weight::default()
	}
}
