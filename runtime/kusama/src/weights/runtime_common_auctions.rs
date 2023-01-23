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
//! Autogenerated weights for `runtime_common::auctions`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=runtime_common::auctions
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_common_auctions.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::auctions`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::auctions::WeightInfo for WeightInfo<T> {
	// Storage: Auctions AuctionInfo (r:1 w:1)
	// Storage: Auctions AuctionCounter (r:1 w:1)
	fn new_auction() -> Weight {
		// Minimum execution time: 16_831 nanoseconds.
		Weight::from_ref_time(17_389_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Paras ParaLifecycles (r:1 w:0)
	// Storage: Auctions AuctionCounter (r:1 w:0)
	// Storage: Auctions AuctionInfo (r:1 w:0)
	// Storage: Slots Leases (r:1 w:0)
	// Storage: Auctions Winning (r:1 w:1)
	// Storage: Auctions ReservedAmounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn bid() -> Weight {
		// Minimum execution time: 72_130 nanoseconds.
		Weight::from_ref_time(73_572_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Auctions AuctionInfo (r:1 w:1)
	// Storage: Babe NextRandomness (r:1 w:0)
	// Storage: Babe EpochStart (r:1 w:0)
	// Storage: Auctions AuctionCounter (r:1 w:0)
	// Storage: Auctions Winning (r:3600 w:3600)
	// Storage: Auctions ReservedAmounts (r:37 w:36)
	// Storage: System Account (r:36 w:36)
	// Storage: Slots Leases (r:7 w:7)
	// Storage: Paras ParaLifecycles (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Registrar Paras (r:1 w:1)
	fn on_initialize() -> Weight {
		// Minimum execution time: 15_625_499 nanoseconds.
		Weight::from_ref_time(15_950_054_000)
			.saturating_add(T::DbWeight::get().reads(3688))
			.saturating_add(T::DbWeight::get().writes(3683))
	}
	// Storage: Auctions ReservedAmounts (r:37 w:36)
	// Storage: System Account (r:36 w:36)
	// Storage: Auctions Winning (r:0 w:3600)
	// Storage: Auctions AuctionInfo (r:0 w:1)
	fn cancel_auction() -> Weight {
		// Minimum execution time: 4_619_056 nanoseconds.
		Weight::from_ref_time(4_709_611_000)
			.saturating_add(T::DbWeight::get().reads(73))
			.saturating_add(T::DbWeight::get().writes(3673))
	}
}
