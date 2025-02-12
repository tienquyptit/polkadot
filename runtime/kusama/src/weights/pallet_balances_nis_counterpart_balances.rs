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
//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_balances
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn transfer_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `11258`
		// Minimum execution time: 39_224_000 picoseconds.
		Weight::from_parts(39_636_000, 0)
			.saturating_add(Weight::from_parts(0, 11258))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `9757`
		// Minimum execution time: 27_245_000 picoseconds.
		Weight::from_parts(28_139_000, 0)
			.saturating_add(Weight::from_parts(0, 9757))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn force_set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `5078`
		// Minimum execution time: 16_568_000 picoseconds.
		Weight::from_parts(16_964_000, 0)
			.saturating_add(Weight::from_parts(0, 5078))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn force_set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `393`
		//  Estimated: `8671`
		// Minimum execution time: 24_153_000 picoseconds.
		Weight::from_parts(24_529_000, 0)
			.saturating_add(Weight::from_parts(0, 8671))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322`
		//  Estimated: `13861`
		// Minimum execution time: 41_412_000 picoseconds.
		Weight::from_parts(42_310_000, 0)
			.saturating_add(Weight::from_parts(0, 13861))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `9757`
		// Minimum execution time: 35_218_000 picoseconds.
		Weight::from_parts(36_321_000, 0)
			.saturating_add(Weight::from_parts(0, 9757))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `3577`
		// Minimum execution time: 14_706_000 picoseconds.
		Weight::from_parts(15_135_000, 0)
			.saturating_add(Weight::from_parts(0, 3577))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn upgrade_accounts(_: u32) -> Weight {
		Weight::from_parts(0, 0)
	}
}
