use ibc::core::{
	ics24_host::identifier::PortId,
	ics26_routing::context::{Module, ModuleId},
};
use pallet_ibc::{
	light_client_common::RelayChain, routing::ModuleRouter, DenomToAssetId, IbcAssetIds, IbcAssets,
	IbcDenoms,
};


#[allow(clippy::derivable_impls)]
impl Default for Runtime {
	fn default() -> Self {
		Self {}
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Router {
	pallet_ibc_ping: pallet_ibc_ping::IbcModule<Runtime>,
	pallet_cosmwasm: CosmwasmRouter,
}

impl ModuleRouter for Router {
	fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module> {
		match module_id.as_ref() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.pallet_ibc_ping),
			_ => self.pallet_cosmwasm.get_route_mut(module_id),
		}
	}

	fn has_route(module_id: &ModuleId) -> bool {
		matches!(module_id.as_ref(), pallet_ibc_ping::MODULE_ID) ||
			CosmwasmRouter::has_route(module_id)
	}

	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId> {
		match port_id.as_str() {
			pallet_ibc_ping::PORT_ID => ModuleId::from_str(pallet_ibc_ping::MODULE_ID).ok(),
			_ => CosmwasmRouter::lookup_module_by_port(port_id),
		}
	}
}


pub struct IbcDenomToAssetIdConversion;

impl DenomToAssetId<Runtime> for IbcDenomToAssetIdConversion {
	type Error = DispatchError;

	fn from_denom_to_asset_id(denom: &String) -> Result<CurrencyId, Self::Error> {
		let denom_bytes = denom.as_bytes().to_vec();
		if let Some(id) = IbcDenoms::<Runtime>::get(&denom_bytes) {
			return Ok(id)
		}

		let asset_id =
			<currency_factory::Pallet<Runtime> as CurrencyFactoryT>::create(RangeId::IBC_ASSETS)?;

		IbcDenoms::<Runtime>::insert(denom_bytes.clone(), asset_id);
		IbcAssetIds::<Runtime>::insert(asset_id, denom_bytes);

		let location = XcmAssetLocation::new(MultiLocation::new(
			1,
			X1(Junction::GeneralIndex(asset_id.into())),
		));
		assets_registry::Pallet::<Runtime>::set_reserve_location(
			asset_id,
			location,
			Rational64::one(),
			Some(12),
		)?;

		Ok(asset_id)
	}

	fn from_asset_id_to_denom(id: CurrencyId) -> Option<String> {
		IbcAssetIds::<Runtime>::get(id).and_then(|denom| String::from_utf8(denom).ok())
	}

	fn ibc_assets(start_key: Option<Either<CurrencyId, u32>>, limit: u64) -> IbcAssets<CurrencyId> {
		let mut iterator = match start_key {
			None => IbcAssetIds::<Runtime>::iter().skip(0),
			Some(Left(asset_id)) => {
				let raw_key = asset_id.encode();
				IbcAssetIds::<Runtime>::iter_from(raw_key).skip(0)
			},
			Some(Right(offset)) => IbcAssetIds::<Runtime>::iter().skip(offset as usize),
		};

		let denoms = iterator.by_ref().take(limit as usize).map(|(_, denom)| denom).collect();
		let maybe_currency_id = iterator.next().map(|(id, ..)| id);
		IbcAssets {
			denoms,
			total_count: IbcAssetIds::<Runtime>::count() as u64,
			next_id: maybe_currency_id,
		}
	}
}

impl pallet_ibc::Config for Runtime {
	type TimeProvider = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type NativeCurrency = Balances;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type NativeAssetId = NativeAssetId;
	type IbcDenomToAssetIdConversion = IbcDenomToAssetIdConversion;
	const PALLET_PREFIX: &'static [u8] = b"ibc/";
	const LIGHT_CLIENT_PROTOCOL: pallet_ibc::LightClientProtocol =
		pallet_ibc::LightClientProtocol::Grandpa;
	type AccountIdConversion = ibc_primitives::IbcAccount<AccountId>;
	type Fungibles = Assets;
	type ExpectedBlockTime = ExpectedBlockTime;
	type Router = Router;
	type MinimumConnectionDelay = MinimumConnectionDelay;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type SentryOrigin = EnsureRoot<AccountId>;
	type SpamProtectionDeposit = SpamProtectionDeposit;
}


impl ibc_runtime_api::IbcRuntimeApi<Block, CurrencyId> for Runtime {
    fn para_id() -> u32 {
        <Runtime as cumulus_pallet_parachain_system::Config>::SelfParaId::get().into()
    }

    fn child_trie_key() -> Vec<u8> {
        <Runtime as pallet_ibc::Config>::PALLET_PREFIX.to_vec()
    }

    fn query_balance_with_address(addr: Vec<u8>) -> Option<u128> {
        Ibc::query_balance_with_address(addr).ok()
    }

    fn query_send_packet_info(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::PacketInfo>> {
        Ibc::get_send_packet_info(channel_id, port_id, seqs).ok()
    }

    fn query_recv_packet_info(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::PacketInfo>> {
        Ibc::get_recv_packet_info(channel_id, port_id, seqs).ok()
    }

    fn client_update_time_and_height(client_id: Vec<u8>, revision_number: u64, revision_height: u64) -> Option<(u64, u64)>{
        Ibc::client_update_time_and_height(client_id, revision_number, revision_height).ok()
    }

    fn client_state(client_id: Vec<u8>) -> Option<ibc_primitives::QueryClientStateResponse> {
        Ibc::client(client_id).ok()
    }

    fn client_consensus_state(client_id: Vec<u8>, revision_number: u64, revision_height: u64, latest_cs: bool) -> Option<ibc_primitives::QueryConsensusStateResponse> {
        Ibc::consensus_state(client_id, revision_number, revision_height, latest_cs).ok()
    }

    fn clients() -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
        Some(Ibc::clients())
    }

    fn connection(connection_id: Vec<u8>) -> Option<ibc_primitives::QueryConnectionResponse>{
        Ibc::connection(connection_id).ok()
    }

    fn connections() -> Option<ibc_primitives::QueryConnectionsResponse> {
        Ibc::connections().ok()
    }

    fn connection_using_client(client_id: Vec<u8>) -> Option<Vec<ibc_primitives::IdentifiedConnection>>{
        Ibc::connection_using_client(client_id).ok()
    }

    fn connection_handshake(client_id: Vec<u8>, connection_id: Vec<u8>) -> Option<ibc_primitives::ConnectionHandshake> {
        Ibc::connection_handshake(client_id, connection_id).ok()
    }

    fn channel(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryChannelResponse> {
        Ibc::channel(channel_id, port_id).ok()
    }

    fn channel_client(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::IdentifiedClientState> {
        Ibc::channel_client(channel_id, port_id).ok()
    }

    fn connection_channels(connection_id: Vec<u8>) -> Option<ibc_primitives::QueryChannelsResponse> {
        Ibc::connection_channels(connection_id).ok()
    }

    fn channels() -> Option<ibc_primitives::QueryChannelsResponse> {
        Ibc::channels().ok()
    }

    fn packet_commitments(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryPacketCommitmentsResponse> {
        Ibc::packet_commitments(channel_id, port_id).ok()
    }

    fn packet_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryPacketAcknowledgementsResponse>{
        Ibc::packet_acknowledgements(channel_id, port_id).ok()
    }

    fn unreceived_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>> {
        Ibc::unreceived_packets(channel_id, port_id, seqs).ok()
    }

    fn unreceived_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>> {
        Ibc::unreceived_acknowledgements(channel_id, port_id, seqs).ok()
    }

    fn next_seq_recv(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryNextSequenceReceiveResponse> {
        Ibc::next_seq_recv(channel_id, port_id).ok()
    }

    fn packet_commitment(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketCommitmentResponse> {
        Ibc::packet_commitment(channel_id, port_id, seq).ok()
    }

    fn packet_acknowledgement(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketAcknowledgementResponse> {
        Ibc::packet_acknowledgement(channel_id, port_id, seq).ok()
    }

    fn packet_receipt(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketReceiptResponse> {
        Ibc::packet_receipt(channel_id, port_id, seq).ok()
    }

    fn denom_trace(asset_id: CurrencyId) -> Option<ibc_primitives::QueryDenomTraceResponse> {
        Ibc::get_denom_trace(asset_id)
    }

    fn denom_traces(key: Option<CurrencyId>, offset: Option<u32>, limit: u64, count_total: bool) -> ibc_primitives::QueryDenomTracesResponse {
        let key = key.map(Either::Left).or_else(|| offset.map(Either::Right));
        Ibc::get_denom_traces(key, limit, count_total)
    }

    fn block_events(extrinsic_index: Option<u32>) -> Vec<Result<pallet_ibc::events::IbcEvent, pallet_ibc::errors::IbcError>> {
        let mut raw_events = frame_system::Pallet::<Self>::read_events_no_consensus().into_iter();
        if let Some(idx) = extrinsic_index {
            raw_events.find_map(|e| {
                let frame_system::EventRecord{ event, phase, ..} = *e;
                match (event, phase) {
                    (RuntimeEvent::Ibc(pallet_ibc::Event::Events{ events }), frame_system::Phase::ApplyExtrinsic(index)) if index == idx => Some(events),
                    _ => None
                }
            }).unwrap_or_default()
        }
        else {
            raw_events.filter_map(|e| {
                let frame_system::EventRecord{ event, ..} = *e;

                match event {
                    RuntimeEvent::Ibc(pallet_ibc::Event::Events{ events }) => {
                            Some(events)
                        },
                    _ => None
                }
            }).flatten().collect()
        }
    }
}
