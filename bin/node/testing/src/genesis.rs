//! Genesis Configuration.

use crate::keyring::*;
use node_runtime::constants::currency::*;
use node_runtime::{
	AccountId, BalancesConfig, ContractsConfig, GenesisConfig, GrandpaConfig, IndicesConfig, SessionConfig,
	SocietyConfig, StakingConfig, SystemConfig, WASM_BINARY,
};
use sp_core::ChangesTrieConfiguration;
use sp_keyring::{Ed25519Keyring, Sr25519Keyring};
use sp_runtime::Perbill;

/// Create genesis runtime configuration for tests.
pub fn config(support_changes_trie: bool, code: Option<&[u8]>) -> GenesisConfig {
	config_endowed(support_changes_trie, code, Default::default())
}

/// Create genesis runtime configuration for tests with some extra
/// endowed accounts.
pub fn config_endowed(support_changes_trie: bool, code: Option<&[u8]>, extra_endowed: Vec<AccountId>) -> GenesisConfig {
	let mut endowed = vec![
		(alice(), 111 * DOLLARS),
		(bob(), 100 * DOLLARS),
		(charlie(), 100_000_000 * DOLLARS),
		(dave(), 111 * DOLLARS),
		(eve(), 101 * DOLLARS),
		(ferdie(), 100 * DOLLARS),
	];

	endowed.extend(extra_endowed.into_iter().map(|endowed| (endowed, 100 * DOLLARS)));

	GenesisConfig {
		frame_system: Some(SystemConfig {
			changes_trie_config: if support_changes_trie {
				Some(ChangesTrieConfiguration {
					digest_interval: 2,
					digest_levels: 2,
				})
			} else {
				None
			},
			code: code.map(|x| x.to_vec()).unwrap_or_else(|| WASM_BINARY.to_vec()),
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_balances: Some(BalancesConfig { balances: endowed }),
		pallet_session: Some(SessionConfig {
			keys: vec![
				(
					dave(),
					alice(),
					to_session_keys(&Ed25519Keyring::Alice, &Sr25519Keyring::Alice),
				),
				(
					eve(),
					bob(),
					to_session_keys(&Ed25519Keyring::Bob, &Sr25519Keyring::Bob),
				),
				(
					ferdie(),
					charlie(),
					to_session_keys(&Ed25519Keyring::Charlie, &Sr25519Keyring::Charlie),
				),
			],
		}),
		pallet_staking: Some(StakingConfig {
			current_era: 0,
			stakers: vec![
				(dave(), alice(), 111 * DOLLARS, pallet_staking::StakerStatus::Validator),
				(eve(), bob(), 100 * DOLLARS, pallet_staking::StakerStatus::Validator),
				(
					ferdie(),
					charlie(),
					100 * DOLLARS,
					pallet_staking::StakerStatus::Validator,
				),
			],
			validator_count: 3,
			minimum_validator_count: 0,
			slash_reward_fraction: Perbill::from_percent(10),
			invulnerables: vec![alice(), bob(), charlie()],
			..Default::default()
		}),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		pallet_babe: Some(Default::default()),
		pallet_grandpa: Some(GrandpaConfig { authorities: vec![] }),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(Default::default()),
		pallet_democracy: Some(Default::default()),
		pallet_collective_Instance1: Some(Default::default()),
		pallet_collective_Instance2: Some(Default::default()),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_sudo: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_society: Some(SocietyConfig {
			members: vec![alice(), bob()],
			pot: 0,
			max_members: 999,
		}),
		pallet_vesting: Some(Default::default()),
	}
}
