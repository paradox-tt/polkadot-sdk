// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_sudo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-yprdrvc7-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_sudo
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/sudo/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_sudo`.
pub trait WeightInfo {
	fn set_key() -> Weight;
	fn sudo() -> Weight;
	fn sudo_as() -> Weight;
}

/// Weights for `pallet_sudo` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `1517`
		// Minimum execution time: 9_537_000 picoseconds.
		Weight::from_parts(9_842_000, 1517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `1517`
		// Minimum execution time: 9_820_000 picoseconds.
		Weight::from_parts(10_099_000, 1517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `1517`
		// Minimum execution time: 9_848_000 picoseconds.
		Weight::from_parts(10_101_000, 1517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `1517`
		// Minimum execution time: 9_537_000 picoseconds.
		Weight::from_parts(9_842_000, 1517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `1517`
		// Minimum execution time: 9_820_000 picoseconds.
		Weight::from_parts(10_099_000, 1517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `1517`
		// Minimum execution time: 9_848_000 picoseconds.
		Weight::from_parts(10_101_000, 1517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}
