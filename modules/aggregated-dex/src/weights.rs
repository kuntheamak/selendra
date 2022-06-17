// Copyright 2021-2022 Selendra.
// This file is part of Selendra.

// Selendra is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Selendra is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Selendra.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for module_aggregated_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/selendra
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_aggregated_dex
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/aggregated-dex/src/weights.rs
// --template=./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_aggregated_dex.
pub trait WeightInfo {
	fn swap_with_exact_supply(u: u32, ) -> Weight;
	fn update_aggregated_swap_paths(u: u32, ) -> Weight;
}

/// Weights for module_aggregated_dex using the Selendra node and recommended hardware.
pub struct SelendraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SelendraWeight<T> {
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		(93_799_000 as Weight)
			// Standard Error: 117_000
			.saturating_add((16_008_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(u as Weight)))
	}
	fn update_aggregated_swap_paths(u: u32, ) -> Weight {
		(2_268_000 as Weight)
			// Standard Error: 245_000
			.saturating_add((19_990_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(u as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		(93_799_000 as Weight)
			// Standard Error: 117_000
			.saturating_add((16_008_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(u as Weight)))
	}
	fn update_aggregated_swap_paths(u: u32, ) -> Weight {
		(2_268_000 as Weight)
			// Standard Error: 245_000
			.saturating_add((19_990_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(u as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(u as Weight)))
	}
}
