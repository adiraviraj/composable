
//! Autogenerated weights for `scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `3a6013dfb40d`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/jif3kmz9kgiwz8hg8nzb9d2kiga1rnga-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> scheduler::WeightInfo for WeightInfo<T> {
	/// Storage: Scheduler IncompleteSince (r:1 w:1)
	/// Proof: Scheduler IncompleteSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn service_agendas_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31`
		//  Estimated: `499`
		// Minimum execution time: 5_977 nanoseconds.
		Weight::from_ref_time(6_327_000)
			.saturating_add(Weight::from_proof_size(499))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110 + s * (177 ±0)`
		//  Estimated: `41438`
		// Minimum execution time: 5_831 nanoseconds.
		Weight::from_ref_time(11_560_047)
			.saturating_add(Weight::from_proof_size(41438))
			// Standard Error: 6_482
			.saturating_add(Weight::from_ref_time(1_770_943).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_125 nanoseconds.
		Weight::from_ref_time(11_529_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `245 + s * (1 ±0)`
		//  Estimated: `5286 + s * (1 ±0)`
		// Minimum execution time: 35_809 nanoseconds.
		Weight::from_ref_time(35_972_000)
			.saturating_add(Weight::from_proof_size(5286))
			// Standard Error: 14
			.saturating_add(Weight::from_ref_time(2_063).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(1).saturating_mul(s.into()))
	}
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn service_task_named() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_717 nanoseconds.
		Weight::from_ref_time(15_247_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_periodic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_334 nanoseconds.
		Weight::from_ref_time(11_582_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	fn execute_dispatch_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `2687`
		// Minimum execution time: 11_101 nanoseconds.
		Weight::from_ref_time(11_590_000)
			.saturating_add(Weight::from_proof_size(2687))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_466 nanoseconds.
		Weight::from_ref_time(5_661_000)
			.saturating_add(Weight::from_proof_size(0))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110 + s * (177 ±0)`
		//  Estimated: `41438`
		// Minimum execution time: 23_702 nanoseconds.
		Weight::from_ref_time(29_122_003)
			.saturating_add(Weight::from_proof_size(41438))
			// Standard Error: 9_756
			.saturating_add(Weight::from_ref_time(1_889_230).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110 + s * (177 ±0)`
		//  Estimated: `41438`
		// Minimum execution time: 30_803 nanoseconds.
		Weight::from_ref_time(29_265_214)
			.saturating_add(Weight::from_proof_size(41438))
			// Standard Error: 10_536
			.saturating_add(Weight::from_ref_time(3_165_472).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287 + s * (185 ±0)`
		//  Estimated: `43961`
		// Minimum execution time: 29_496 nanoseconds.
		Weight::from_ref_time(36_424_407)
			.saturating_add(Weight::from_proof_size(43961))
			// Standard Error: 12_235
			.saturating_add(Weight::from_ref_time(1_945_220).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `313 + s * (185 ±0)`
		//  Estimated: `43961`
		// Minimum execution time: 32_975 nanoseconds.
		Weight::from_ref_time(33_734_720)
			.saturating_add(Weight::from_proof_size(43961))
			// Standard Error: 10_246
			.saturating_add(Weight::from_ref_time(3_149_868).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
