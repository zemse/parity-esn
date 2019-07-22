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

//! Fully verified transaction

use std::ops;
use ethjson;
use ethereum_types::Address;
use parity_util_mem::MallocSizeOf;
use rlp;
use ethkey::{Secret, Public};
use crate::transaction::{
	Transaction,
	UnverifiedTransaction,
	Action,
};

/// Fully verified ethereum transation.
#[derive(Debug, Clone, Eq, PartialEq, MallocSizeOf)]
pub struct VerifiedTransaction {
	pub transaction: UnverifiedTransaction,
	pub sender: Address,
	pub public: Option<Public>,
}

impl From<ethjson::state::Transaction> for VerifiedTransaction {
	fn from(t: ethjson::state::Transaction) -> Self {
		let to: Option<ethjson::hash::Address> = t.to.into();
		let secret = t.secret.map(|s| Secret::from(s.0));
		let tx = Transaction {
			nonce: t.nonce.into(),
			gas_price: t.gas_price.into(),
			gas: t.gas_limit.into(),
			action: match to {
				Some(to) => Action::Call(to.into()),
				None => Action::Create
			},
			value: t.value.into(),
			data: t.data.into(),
		};
		match secret {
			Some(s) => tx.sign(&s, None),
			None => tx.null_sign(1),
		}
	}
}

impl rlp::Encodable for VerifiedTransaction {
	fn rlp_append(&self, s: &mut rlp::RlpStream) {
		s.append_raw(&self.rlp, 1);
	}
}

impl ops::Deref for VerifiedTransaction {
	type Target = UnverifiedTransaction;
	fn deref(&self) -> &Self::Target {
		&self.transaction
	}
}

impl From<VerifiedTransaction> for UnverifiedTransaction {
	fn from(tx: VerifiedTransaction) -> Self {
		tx.transaction
	}
}
