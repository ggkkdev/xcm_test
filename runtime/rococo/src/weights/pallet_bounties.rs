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
//! Autogenerated weights for `pallet_bounties`
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
// --pallet=pallet_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bounties::WeightInfo for WeightInfo<T> {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	/// The range of component `d` is `[0, 16384]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Minimum execution time: 29_855 nanoseconds.
		Weight::from_ref_time(32_084_917)
			// Standard Error: 8
			.saturating_add(Weight::from_ref_time(877).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	fn approve_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn propose_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn unassign_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn accept_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn award_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn claim_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		// Minimum execution time: 47_203 nanoseconds.
		Weight::from_ref_time(48_590_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn close_bounty_active() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn extend_bounty_expiry() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(_b: u32, ) -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(3_564_251)
	}
}
