// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `frame_benchmarking::baseline`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=frame_benchmarking::baseline
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/frame_benchmarking_baseline.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_benchmarking::baseline`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_benchmarking::baseline::WeightInfo for WeightInfo<T> {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		// Minimum execution time: 111 nanoseconds.
		Weight::from_ref_time(125_746)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		// Minimum execution time: 108 nanoseconds.
		Weight::from_ref_time(127_834)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		// Minimum execution time: 110 nanoseconds.
		Weight::from_ref_time(133_152)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		// Minimum execution time: 110 nanoseconds.
		Weight::from_ref_time(128_930)
	}
	/// The range of component `i` is `[0, 100]`.
	fn hashing(_i: u32, ) -> Weight {
		// Minimum execution time: 23_879_119 nanoseconds.
		Weight::from_ref_time(23_953_239_689)
	}
	/// The range of component `i` is `[0, 100]`.
	fn sr25519_verification(i: u32, ) -> Weight {
		// Minimum execution time: 136 nanoseconds.
		Weight::from_ref_time(2_453_545)
			// Standard Error: 6_526
			.saturating_add(Weight::from_ref_time(55_601_782).saturating_mul(i.into()))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_read(i: u32, ) -> Weight {
		// Minimum execution time: 121 nanoseconds.
		Weight::from_ref_time(127_000)
			// Standard Error: 8_321
			.saturating_add(Weight::from_ref_time(2_538_099).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_write(i: u32, ) -> Weight {
		// Minimum execution time: 115 nanoseconds.
		Weight::from_ref_time(122_000)
			// Standard Error: 1_010
			.saturating_add(Weight::from_ref_time(388_054).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
}
