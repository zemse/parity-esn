// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::fmt;
use errors::ExecutionError;
use vm;

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone)]
pub enum CallError {
	/// Couldn't find the transaction in the chain.
	TransactionNotFound,
	/// Couldn't find requested block's state in the chain.
	StatePruned,
	/// Couldn't find an amount of gas that didn't result in an exception.
	Exceptional(vm::Error),
	/// Corrupt state.
	StateCorrupt,
	/// Error executing.
	Execution(ExecutionError),
}

impl From<ExecutionError> for CallError {
	fn from(error: ExecutionError) -> Self {
		CallError::Execution(error)
	}
}

impl fmt::Display for CallError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::CallError::*;
		let msg = match *self {
			TransactionNotFound => "Transaction couldn't be found in the chain".into(),
			StatePruned => "Couldn't find the transaction block's state in the chain".into(),
			Exceptional(ref e) => format!("An exception ({}) happened in the execution", e),
			StateCorrupt => "Stored state found to be corrupted.".into(),
			Execution(ref e) => format!("{}", e),
		};

		f.write_fmt(format_args!("Transaction execution error ({}).", msg))
	}
}
