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

use crate::{
	AccountId, BalancesConfig, RuntimeGenesisConfig, SudoConfig,
	SessionConfig, ValidatorSetConfig, SessionKeys, Signature, 
	configs::SS58Prefix
};
use alloc::{vec, vec::Vec, format, string::String};
use serde_json::Value;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_genesis_builder::{self, PresetId};
use sp_keyring::AccountKeyring;
#[allow(unused_imports)]
use sp_core::{ecdsa,sr25519};
use sp_core::{Pair, Public};
use sp_runtime::traits::{ Verify, IdentifyAccount };

type Properties = serde_json::map::Map<String, serde_json::Value>;

fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

#[allow(dead_code)]
type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
/// For use with `AccountId32`, `dead_code` if `AccountId20`.
#[allow(dead_code)]
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId) {
	(
		get_account_id_from_seed::<ecdsa::Public>(s),
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s)
	)
}

fn properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());
	properties
}

// Returns the genesis config presets populated with given parameters.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
	root: AccountId,
) -> Value {
	let config = RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1u128 << 60))
				.collect::<Vec<_>>(),
		},
		validator_set: ValidatorSetConfig {
			initial_validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},
		session: SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone()))
			}).collect::<Vec<_>>(),
			non_authority_keys: vec![],
		},
		aura: pallet_aura::GenesisConfig {
			authorities: vec![], //initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>(),
		},
		grandpa: pallet_grandpa::GenesisConfig {
			authorities: vec![], //initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>(),
			..Default::default()
		},
		sudo: SudoConfig { key: Some(root) },
		..Default::default()
	};

	serde_json::to_value(config).expect("Could not build genesis config.")
}

/// Return the development genesis config.
pub fn development_config_genesis() -> Value {
	testnet_genesis(
		vec![(
			AccountKeyring::Alice.to_account_id().into(),
			sp_keyring::Sr25519Keyring::Alice.public().into(),
			sp_keyring::Ed25519Keyring::Alice.public().into(),
		)],
		vec![
			AccountKeyring::Alice.to_account_id().into(),
			AccountKeyring::Bob.to_account_id().into(),
		],
		sp_keyring::AccountKeyring::Alice.to_account_id().into(),
	)
}

/// Return the local genesis config preset.
pub fn local_config_genesis() -> Value {
	testnet_genesis(
		vec![
			(
				AccountKeyring::Alice.to_account_id().into(),
				sp_keyring::Sr25519Keyring::Alice.public().into(),
				sp_keyring::Ed25519Keyring::Alice.public().into(),
			),
			(
				AccountKeyring::Bob.to_account_id().into(),
				sp_keyring::Sr25519Keyring::Bob.public().into(),
				sp_keyring::Ed25519Keyring::Bob.public().into(),
			),
		],
		AccountKeyring::iter()
			.filter(|v| v != &AccountKeyring::One && v != &AccountKeyring::Two)
			.map(|v| v.to_account_id().into())
			.collect::<Vec<_>>(),
		AccountKeyring::Alice.to_account_id().into(),
	)
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
	]
}
