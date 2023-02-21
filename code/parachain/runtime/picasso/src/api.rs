impl_runtime_apis! {
	impl assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance, ForeignAssetId> for Runtime {
		fn balance_of(SafeRpcWrapper(asset_id): SafeRpcWrapper<CurrencyId>, account_id: AccountId) -> SafeRpcWrapper<Balance> /* Balance */ {
			SafeRpcWrapper(<Assets as fungibles::Inspect::<AccountId>>::balance(asset_id, &account_id))
		}

		fn list_assets() -> Vec<Asset<Balance, ForeignAssetId>> {
			// Hardcoded assets
			let assets = CurrencyId::list_assets().into_iter().map(|mut asset| {
				// Add hardcoded ratio and ED for well known assets
				asset.ratio = WellKnownForeignToNativePriceConverter::get_ratio(CurrencyId(asset.id));
				asset.existential_deposit = multi_existential_deposits::<AssetsRegistry>(&asset.id.into());
				asset
			}).collect::<Vec<_>>();

			// Assets from the assets-registry pallet
			let foreign_assets = assets_registry::Pallet::<Runtime>::get_foreign_assets_list();

			// Override asset data for hardcoded assets that have been manually updated, and append
			// new assets without duplication
			foreign_assets.into_iter().fold(assets, |mut acc, mut foreign_asset| {
				if let Some(asset) = acc.iter_mut().find(|asset_i| asset_i.id == foreign_asset.id) {
					// Update asset with data from assets-registry
					asset.decimals = foreign_asset.decimals;
					asset.foreign_id = foreign_asset.foreign_id.clone();
					asset.ratio = foreign_asset.ratio;
				} else {
					foreign_asset.existential_deposit = multi_existential_deposits::<AssetsRegistry>(&foreign_asset.id.into());
					acc.push(foreign_asset.clone())
				}
				acc
			})
		}
	}

	impl crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi<Block, AccountId, Balance> for Runtime {
		fn amount_available_to_claim_for(account_id: AccountId) -> SafeRpcWrapper<Balance> {
			SafeRpcWrapper (
				crowdloan_rewards::amount_available_to_claim_for::<Runtime>(account_id)
					.unwrap_or_else(|_| Balance::zero())
			)
		}
	}

	impl pablo_runtime_api::PabloRuntimeApi<Block, AccountId, PoolId, CurrencyId, Balance> for Runtime {
		fn prices_for(
			pool_id: PoolId,
			base_asset_id: CurrencyId,
			quote_asset_id: CurrencyId,
			amount: Balance
		) -> PriceAggregate<SafeRpcWrapper<PoolId>, SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>> {
			pablo::prices_for::<Runtime>(
				pool_id,
				base_asset_id,
				quote_asset_id,
				amount
			)
			.map(|p| PriceAggregate{
				pool_id: SafeRpcWrapper(p.pool_id),
				base_asset_id: SafeRpcWrapper(p.base_asset_id),
				quote_asset_id: SafeRpcWrapper(p.quote_asset_id),
				spot_price: SafeRpcWrapper(p.spot_price)
			})
			.unwrap_or_else(|_| PriceAggregate{
				pool_id: SafeRpcWrapper(pool_id),
				base_asset_id: SafeRpcWrapper(base_asset_id),
				quote_asset_id: SafeRpcWrapper(quote_asset_id),
				spot_price: SafeRpcWrapper(0_u128)
			})
		}

		fn simulate_add_liquidity(
			who: SafeRpcWrapper<AccountId>,
			pool_id: SafeRpcWrapper<PoolId>,
			amounts: BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>,
		) -> SafeRpcWrapper<Balance> {
			SafeRpcWrapper(
				<Pablo as Amm>::simulate_add_liquidity(
					&who.0,
					pool_id.0,
					amounts.iter().map(|(k, v)| (k.0, v.0)).collect(),
				)
				.unwrap_or_else(|_| Zero::zero())
			)
		}

		fn simulate_remove_liquidity(
			who: SafeRpcWrapper<AccountId>,
			pool_id: SafeRpcWrapper<PoolId>,
			lp_amount: SafeRpcWrapper<Balance>,
			min_expected_amounts: BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>,
		) -> BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>> {
			<Pablo as Amm>::simulate_remove_liquidity(
				&who.0,
				pool_id.0,
				lp_amount.0,
				min_expected_amounts
					.iter()
					.map(|(k, v)| (k.0, v.0))
					.collect()
				)
				.map(|simulation_result| {
					simulation_result
						.into_iter()
						.map(|(k, v)| (SafeRpcWrapper(k), SafeRpcWrapper(v)))
						.collect()
				})
				.unwrap_or_default()
		}
	}

	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	impl system_rpc_runtime_api::AccountNonceApi<Block, AccountId, AccountIndex> for Runtime {
		fn account_nonce(account: AccountId) -> AccountIndex {
			System::account_nonce(account)
		}
	}

	impl transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}


	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use session_benchmarking::Pallet as SessionBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);
			let storage_info = AllPalletsWithSystem::storage_info();
			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			use session_benchmarking::Pallet as SessionBench;
			impl session_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}