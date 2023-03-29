use common::{AccountId, AuraId, Balance};
use composable_runtime::GenesisConfig;

use super::{Extensions, ParaId};

// The block number until ed25519-dalek should be used for signature verification. Decided at
// 1_393_300
pub const DALEK_END_BLOCK: u32 = 1_681_300;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn session_keys(keys: AuraId) -> composable_runtime::opaque::SessionKeys {
	composable_runtime::opaque::SessionKeys { aura: keys }
}

/// Generates the genesis config
pub fn genesis_config(
	root: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	accounts: Vec<AccountId>,
	id: ParaId,
	existential_deposit: Balance,
	treasury: AccountId,
) -> composable_runtime::GenesisConfig {
	composable_runtime::GenesisConfig {
		system: composable_runtime::SystemConfig {
			code: composable_runtime::WASM_BINARY_V2
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: composable_runtime::BalancesConfig {
			// Configure endowed accounts with initial balance.
			balances: vec![
				vec![(treasury, existential_deposit)],
				accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
			]
			.concat(),
		},
		aura: Default::default(),
		sudo: composable_runtime::SudoConfig { key: Some(root.clone()) },
		indices: composable_runtime::IndicesConfig { indices: vec![] },
		parachain_info: composable_runtime::ParachainInfoConfig { parachain_id: id },
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		session: composable_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),        // account id
						acc,                // validator id
						session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		collator_selection: composable_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: existential_deposit * 16,
			..Default::default()
		},
		council_membership: Default::default(),
		council: Default::default(),
		technical_committee: Default::default(),
		technical_committee_membership: composable_runtime::TechnicalCommitteeMembershipConfig {
			members: vec![root.clone()].try_into().expect("const"),
			phantom: Default::default(),
		},
		democracy: Default::default(),
		treasury: Default::default(),
		relayer_xcm: Default::default(),
		assets_registry: composable_runtime::AssetsRegistryConfig {
			assets: primitives::topology::Composable::assets(),
			phantom: Default::default(),
		},
		tokens: Default::default(),
		transaction_payment: Default::default(),
		ibc: composable_runtime::IbcConfig {
			assets: vec![pallet_ibc::pallet::AssetConfig {
				id: primitives::currency::CurrencyId::LAYR,
				denom: b"1".to_vec(),
			}],
		},
		release_membership: composable_runtime::ReleaseMembershipConfig {
			members: vec![root].try_into().expect("const"),
			phantom: Default::default(),
		},
		release_committee: Default::default(),
	}
}
