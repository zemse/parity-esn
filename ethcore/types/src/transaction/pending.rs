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

use std::ops;
use crate::BlockNumber;
use crate::transaction::VerifiedTransaction;

/// Transaction activation condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
	/// Valid at this block number or later.
	Number(BlockNumber),
	/// Valid at this unix time or later.
	Timestamp(u64),
}

/// Queued transaction with additional information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingTransaction {
	/// Signed transaction data.
	pub transaction: VerifiedTransaction,
	/// To be activated at this condition. `None` for immediately.
	pub condition: Option<Condition>,
}

impl PendingTransaction {
	/// Create a new pending transaction from signed transaction.
	pub fn new(transaction: VerifiedTransaction, condition: Option<Condition>) -> Self {
		PendingTransaction {
			transaction,
			condition,
		}
	}
}

impl ops::Deref for PendingTransaction {
	type Target = VerifiedTransaction;

	fn deref(&self) -> &VerifiedTransaction {
		&self.transaction
	}
}

impl From<VerifiedTransaction> for PendingTransaction {
	fn from(transaction: VerifiedTransaction) -> Self {
		PendingTransaction {
			transaction,
			condition: None,
		}
	}
}
