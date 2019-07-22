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

//! Ethereum Transactions

mod basic_verified;
mod error;
mod transaction;
mod pending;
mod verified;

pub use self::basic_verified::BasicVerifiedTransaction;
pub use self::error::Error;
pub use self::transaction::*;
pub use self::pending::{PendingTransaction, Condition};
pub use self::verified::{VerifiedTransaction, VerifiedTransaction as SignedTransaction};

use ethereum_types::{Address, H160};
use ethkey::public_to_address;
use parity_util_mem::MallocSizeOf;
use rlp;

/// Fake address for unsigned transactions as defined by EIP-86.
pub const UNSIGNED_SENDER: Address = H160([0xff; 20]);

/// System sender address for internal state updates.
pub const SYSTEM_ADDRESS: Address = H160([
	0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xfe
]);

/// Transaction action type.
#[derive(Debug, Clone, PartialEq, Eq, MallocSizeOf)]
pub enum Action {
	/// Create creates new contract.
	Create,
	/// Calls contract at given address.
	/// In the case of a transfer, this is the receiver's address.'
	Call(Address),
}

impl Default for Action {
	fn default() -> Action {
		Action::Create
	}
}

impl rlp::Decodable for Action {
	fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
		if rlp.is_empty() {
			Ok(Action::Create)
		} else {
			Ok(Action::Call(rlp.as_val()?))
		}
	}
}

impl rlp::Encodable for Action {
	fn rlp_append(&self, s: &mut rlp::RlpStream) {
		match *self {
			Action::Create => s.append_internal(&""),
			Action::Call(ref addr) => s.append_internal(addr),
		};
	}
}

/// Verify basic signature params. Does not attempt sender recovery.
pub fn verify_basic(
	tx: UnverifiedTransaction,
	check_low_s: bool,
	chain_id: Option<u64>,
	allow_empty_signature: bool
) -> Result<BasicVerifiedTransaction, error::Error> {

	// Checks whether the signature has a low 's' value.
	if check_low_s && !(allow_empty_signature && tx.is_unsigned()) && !tx.signature().is_low_s() {
		return Err(ethkey::Error::InvalidSignature.into())
	}

	// Disallow unsigned transactions in case EIP-86 is disabled.
	if !allow_empty_signature && tx.is_unsigned() {
		return Err(ethkey::Error::InvalidSignature.into())
	}

	// EIP-86: Transactions of this form MUST have gasprice = 0, nonce = 0, value = 0, and do NOT increment the nonce of account 0.
	if allow_empty_signature && tx.is_unsigned() && !(tx.gas_price.is_zero() && tx.value.is_zero() && tx.nonce.is_zero()) {
		return Err(ethkey::Error::InvalidSignature.into())
	}

	// Check if this transaction belongs to the currently processed chain.
	if tx.chain_id().is_some() && tx.chain_id() != chain_id {
		return Err(error::Error::InvalidChainId.into())
	}

	let basic_verified = BasicVerifiedTransaction {
		transaction: tx,
	};

	Ok(basic_verified)
}

/// Performs a heavy check of the transaction signature.
pub fn verify_signature(tx: BasicVerifiedTransaction) -> Result<VerifiedTransaction, error::Error> {
	if tx.is_unsigned() {
		Ok(VerifiedTransaction {
			transaction: tx.transaction,
			sender: UNSIGNED_SENDER,
			public: None,
		})
	} else {
		let public = tx.recover_public()?;
		let sender = public_to_address(&public);

		Ok(VerifiedTransaction {
			transaction: tx.transaction,
			sender: sender,
			public: Some(public),
		})
	}
}
