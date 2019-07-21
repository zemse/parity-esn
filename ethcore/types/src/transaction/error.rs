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

use std::error;
use ethereum_types::U256;
use ethkey;
use rlp;
use unexpected::OutOfBounds;

#[derive(Debug, PartialEq, Clone, Display)]
/// Errors concerning transaction processing.
pub enum Error {
	/// Transaction is already imported to the queue
	#[display(fmt = "Already imported")]
	AlreadyImported,
	/// Transaction is not valid anymore (state already has higher nonce)
	#[display(fmt = "No longer valid")]
	Old,
	/// Transaction has too low fee
	/// (there is already a transaction with the same sender-nonce but higher gas price)
	#[display(fmt = "Gas price too low to replace")]
	TooCheapToReplace,
	/// Transaction was not imported to the queue because limit has been reached.
	#[display(fmt = "Transaction limit reached")]
	LimitReached,
	/// Transaction's gas price is below threshold.
	#[display(fmt = "Insufficient gas price. Min = {}, Given = {}", minimal, got)]
	InsufficientGasPrice {
		/// Minimal expected gas price
		minimal: U256,
		/// Transaction gas price
		got: U256,
	},
	/// Transaction's gas is below currently set minimal gas requirement.
	#[display(fmt = "Insufficient gas. Min = {}, Given = {}", minimal, got)]
	InsufficientGas {
		/// Minimal expected gas
		minimal: U256,
		/// Transaction gas
		got: U256,
	},
	/// Sender doesn't have enough funds to pay for this transaction
	#[display(fmt = "Insufficient balance for transaction. Balance = {}, Cost = {}", balance, cost)]
	InsufficientBalance {
		/// Senders balance
		balance: U256,
		/// Transaction cost
		cost: U256,
	},
	/// Transactions gas is higher then current gas limit
	#[display(fmt = "Gas limit exceeded. Limit = {}, Given = {}", limit, got)]
	GasLimitExceeded {
		/// Current gas limit
		limit: U256,
		/// Declared transaction gas
		got: U256,
	},
	/// Transaction's gas limit (aka gas) is invalid.
	#[display(fmt = "Invalid gas limit. {}", _0)]
	InvalidGasLimit(OutOfBounds<U256>),
	/// Transaction sender is banned.
	#[display(fmt = "Sender is temporarily banned.")]
	SenderBanned,
	/// Transaction receipient is banned.
	#[display(fmt = "Recipient is temporarily banned.")]
	RecipientBanned,
	/// Contract creation code is banned.
	#[display(fmt = "Contract code is temporarily banned.")]
	CodeBanned,
	/// Invalid chain ID given.
	#[display(fmt = "Transaction of this chain ID is not allowed on this chain.")]
	InvalidChainId,
	/// Not enough permissions given by permission contract.
	#[display(fmt = "Sender does not have permissions to execute this type of transaction.")]
	NotAllowed,
	/// Signature error
	#[display(fmt = "Transaction has invalid signature: {}.", _0)]
	InvalidSignature(String),
	/// Transaction too big
	#[display(fmt = "Transaction is too big.")]
	TooBig,
	/// Invalid RLP encoding
	#[display(fmt = "Transaction has invalid RLP structure: {}.", _0)]
	InvalidRlp(String),
}

impl From<ethkey::Error> for Error {
	fn from(err: ethkey::Error) -> Self {
		Error::InvalidSignature(format!("{}", err))
	}
}

impl From<rlp::DecoderError> for Error {
	fn from(err: rlp::DecoderError) -> Self {
		Error::InvalidRlp(format!("{}", err))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		"Transaction error"
	}
}
