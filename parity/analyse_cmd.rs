// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! Snapshot and restoration commands.

use std::sync::Arc;

use client_traits::BlockInfo;
use ethcore::client::{DatabaseCompactionProfile, VMType};
use ethcore::miner::Miner;
use ethcore_service::ClientService;
use types::{
	client_types::Mode,
	ids::BlockId,
	encoded,
};

use cache::CacheConfig;
use params::{SpecType, Pruning, Switch, tracing_switch_to_bool, fatdb_switch_to_bool};
use helpers::{to_client_config, execute_upgrades};
use dir::Directories;
use user_defaults::UserDefaults;
use ethcore_private_tx;
use db;

/// Command for snapshot creation or restoration.
#[derive(Debug, PartialEq)]
pub struct AnalyseCommand {
	pub cache_config: CacheConfig,
	pub dirs: Directories,
	pub spec: SpecType,
	pub pruning: Pruning,
	pub pruning_history: u64,
	pub pruning_memory: usize,
	pub tracing: Switch,
	pub fat_db: Switch,
	pub compaction: DatabaseCompactionProfile,
	pub max_round_blocks_to_import: usize,
}

impl AnalyseCommand {
	/// Run the analyser
	pub fn run(self) -> Result<String, String> {
		let service = self.start_service()?;
		let client = service.client();
		let best_bn = client.best_block_header().number();
		println!("> Best block: #{}", best_bn);

		for i in 1..best_bn {
			let block: encoded::Block = match client.block(BlockId::Number(i)) {
				Some(b) => b,
				None => Err(format!("Could not find block #{}", i))?,
			};

			for tx in block.transaction_views() {
				let action_rlp = tx.rlp().at(3).rlp;

				if action_rlp.is_empty() && action_rlp.is_list() {
					println!("Found invalid action for tx {:x} at block #{}", tx.hash(), i);
				}
			}
			if i % 1_000 == 0 {
				println!("... up to block #{} ...", i);
			}
		}

		Ok("OK".to_owned())
	}

	// Start the client service
	fn start_service(self) -> Result<ClientService, String> {
		// load spec file
		let spec = self.spec.spec(&self.dirs.cache)?;

		// load genesis hash
		let genesis_hash = spec.genesis_header().hash();

		// database paths
		let db_dirs = self.dirs.database(genesis_hash, None, spec.data_dir.clone());

		// user defaults path
		let user_defaults_path = db_dirs.user_defaults_path();

		// load user defaults
		let user_defaults = UserDefaults::load(&user_defaults_path)?;

		// select pruning algorithm
		let algorithm = self.pruning.to_algorithm(&user_defaults);

		// check if tracing is on
		let tracing = tracing_switch_to_bool(self.tracing, &user_defaults)?;

		// check if fatdb is on
		let fat_db = fatdb_switch_to_bool(self.fat_db, &user_defaults, algorithm)?;

		// prepare client and snapshot paths.
		let client_path = db_dirs.client_path(algorithm);
		let snapshot_path = db_dirs.snapshot_path();

		// execute upgrades
		execute_upgrades(&self.dirs.base, &db_dirs, algorithm, &self.compaction)?;

		// prepare client config
		let client_config = to_client_config(
			&self.cache_config,
			spec.name.to_lowercase(),
			Mode::Active,
			tracing,
			fat_db,
			self.compaction,
			VMType::default(),
			"".into(),
			algorithm,
			self.pruning_history,
			self.pruning_memory,
			true,
			self.max_round_blocks_to_import,
		);

		let restoration_db_handler = db::restoration_db_handler(&client_path, &client_config);
		let client_db = restoration_db_handler.open(&client_path)
			.map_err(|e| format!("Failed to open database {:?}", e))?;

		let service = ClientService::start(
			client_config,
			&spec,
			client_db,
			&snapshot_path,
			restoration_db_handler,
			&self.dirs.ipc_path(),
			// TODO [ToDr] don't use test miner here
			// (actually don't require miner at all)
			Arc::new(Miner::new_for_tests(&spec, None)),
			Arc::new(ethcore_private_tx::DummySigner),
			Box::new(ethcore_private_tx::NoopEncryptor),
			Default::default(),
			Default::default(),
		).map_err(|e| format!("Client service error: {:?}", e))?;

		Ok(service)
	}
}
