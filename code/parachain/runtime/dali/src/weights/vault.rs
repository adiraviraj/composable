
//! Autogenerated weights for `vault`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-03, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `8934575597cc`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/386hzkyz77l1m76kfsnqr70svvd104hq-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `vault`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> vault::WeightInfo for WeightInfo<T> {
	// Storage: Vault VaultCount (r:1 w:1)
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	fn create() -> Weight {
		// Minimum execution time: 132_269 nanoseconds.
		Weight::from_ref_time(136_063_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		// Minimum execution time: 145_366 nanoseconds.
		Weight::from_ref_time(150_931_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	fn withdraw() -> Weight {
		// Minimum execution time: 127_826 nanoseconds.
		Weight::from_ref_time(131_456_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn emergency_shutdown() -> Weight {
		// Minimum execution time: 35_335 nanoseconds.
		Weight::from_ref_time(35_961_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn start_() -> Weight {
		// Minimum execution time: 35_279 nanoseconds.
		Weight::from_ref_time(37_122_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_surcharge() -> Weight {
		// Minimum execution time: 78_799 nanoseconds.
		Weight::from_ref_time(80_492_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_surcharge() -> Weight {
		// Minimum execution time: 75_343 nanoseconds.
		Weight::from_ref_time(76_574_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	fn delete_tombstoned() -> Weight {
		// Minimum execution time: 35_489 nanoseconds.
		Weight::from_ref_time(36_298_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
