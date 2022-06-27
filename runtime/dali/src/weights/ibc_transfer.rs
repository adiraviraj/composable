
//! Autogenerated weights for `ibc_transfer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `ibc_transfer`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ibc_transfer::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Transfer IbcAssetIds (r:1 w:0)
	// Storage: Ibc ChannelsConnection (r:2 w:0)
	// Storage: Ibc ConnectionClient (r:2 w:0)
	// Storage: Ibc ClientStates (r:1 w:0)
	// Storage: Transfer Params (r:1 w:0)
	// Storage: Ibc Channels (r:1 w:0)
	// Storage: Ibc NextSequenceSend (r:1 w:1)
	// Storage: AssetsRegistry ForeignToLocal (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Ibc Connections (r:1 w:0)
	// Storage: Ibc ConsensusStates (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc PacketCommitment (r:1 w:1)
	// Storage: Ibc CounterForPacketCommitment (r:1 w:1)
	fn transfer() -> Weight {
		(376_893_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(19 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Ibc ChannelCounter (r:1 w:1)
	// Storage: Ibc Connections (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ChannelsConnection (r:1 w:1)
	// Storage: Ibc NextSequenceAck (r:0 w:1)
	// Storage: Ibc NextSequenceSend (r:0 w:1)
	// Storage: Ibc Channels (r:0 w:1)
	// Storage: Ibc NextSequenceRecv (r:0 w:1)
	fn open_channel() -> Weight {
		(132_955_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Transfer Params (r:0 w:1)
	fn set_pallet_params() -> Weight {
		(20_939_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_chan_open_init() -> Weight {
		(654_000 as Weight)
	}
	fn on_chan_open_try() -> Weight {
		(819_000 as Weight)
	}
	// Storage: Transfer ChannelIds (r:1 w:1)
	fn on_chan_open_ack() -> Weight {
		(7_072_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Transfer ChannelIds (r:1 w:1)
	fn on_chan_open_confirm() -> Weight {
		(7_234_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Transfer ChannelIds (r:1 w:1)
	fn on_chan_close_init() -> Weight {
		(8_502_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Transfer ChannelIds (r:1 w:1)
	fn on_chan_close_confirm() -> Weight {
		(8_473_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Transfer Params (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Ibc Acknowledgements (r:1 w:1)
	// Storage: Ibc CounterForAcknowledgements (r:1 w:1)
	fn on_recv_packet() -> Weight {
		(118_912_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn on_acknowledgement_packet() -> Weight {
		(89_973_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn on_timeout_packet() -> Weight {
		(91_642_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}