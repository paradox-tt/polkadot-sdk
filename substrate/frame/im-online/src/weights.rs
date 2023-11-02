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

//! Autogenerated weights for `pallet_im_online`
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
// --pallet=pallet_im_online
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/im-online/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_im_online`.
pub trait WeightInfo {
	fn validate_unsigned_and_then_heartbeat(k: u32, ) -> Weight;
}

/// Weights for `pallet_im_online` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ImOnline::Keys` (r:1 w:0)
	/// Proof: `ImOnline::Keys` (`max_values`: Some(1), `max_size`: Some(320002), added: 320497, mode: `MaxEncodedLen`)
	/// Storage: `ImOnline::ReceivedHeartbeats` (r:1 w:1)
	/// Proof: `ImOnline::ReceivedHeartbeats` (`max_values`: None, `max_size`: Some(25), added: 2500, mode: `MaxEncodedLen`)
	/// Storage: `ImOnline::AuthoredBlocks` (r:1 w:0)
	/// Proof: `ImOnline::AuthoredBlocks` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// The range of component `k` is `[1, 1000]`.
	fn validate_unsigned_and_then_heartbeat(k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `328 + k * (32 ±0)`
		//  Estimated: `321487 + k * (1761 ±0)`
		// Minimum execution time: 78_283_000 picoseconds.
		Weight::from_parts(96_523_256, 321487)
			// Standard Error: 559
			.saturating_add(Weight::from_parts(30_542, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 1761).saturating_mul(k.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ImOnline::Keys` (r:1 w:0)
	/// Proof: `ImOnline::Keys` (`max_values`: Some(1), `max_size`: Some(320002), added: 320497, mode: `MaxEncodedLen`)
	/// Storage: `ImOnline::ReceivedHeartbeats` (r:1 w:1)
	/// Proof: `ImOnline::ReceivedHeartbeats` (`max_values`: None, `max_size`: Some(25), added: 2500, mode: `MaxEncodedLen`)
	/// Storage: `ImOnline::AuthoredBlocks` (r:1 w:0)
	/// Proof: `ImOnline::AuthoredBlocks` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// The range of component `k` is `[1, 1000]`.
	fn validate_unsigned_and_then_heartbeat(k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `328 + k * (32 ±0)`
		//  Estimated: `321487 + k * (1761 ±0)`
		// Minimum execution time: 78_283_000 picoseconds.
		Weight::from_parts(96_523_256, 321487)
			// Standard Error: 559
			.saturating_add(Weight::from_parts(30_542, 0).saturating_mul(k.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 1761).saturating_mul(k.into()))
	}
}
