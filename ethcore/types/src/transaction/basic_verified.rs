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
use parity_util_mem::MallocSizeOf;
use crate::transaction::UnverifiedTransaction;

/// Transaction with verified basic signature params.
#[derive(Debug, Clone, Eq, PartialEq, MallocSizeOf)]
pub struct BasicVerifiedTransaction {
	pub transaction: UnverifiedTransaction,
}

impl ops::Deref for BasicVerifiedTransaction {
	type Target = UnverifiedTransaction;
	fn deref(&self) -> &Self::Target {
		&self.transaction
	}
}
