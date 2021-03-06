// Copyright 2017 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Shareable Polkadot types.

#![warn(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

extern crate rustc_hex;
extern crate byteorder;
#[macro_use]
extern crate crunchy;
#[macro_use]
extern crate fixed_hash;
#[macro_use]
extern crate uint as uint_crate;
extern crate substrate_codec as codec;

#[cfg(feature = "std")]
extern crate serde;
#[cfg(feature = "std")]
extern crate twox_hash;
#[cfg(feature = "std")]
extern crate blake2_rfc;
#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "std")]
extern crate core;
#[cfg(feature = "std")]
extern crate wasmi;

extern crate substrate_runtime_std as rstd;

#[cfg(test)]
extern crate substrate_serializer;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_export]
macro_rules! map {
	($( $name:expr => $value:expr ),*) => (
		vec![ $( ( $name, $value ) ),* ].into_iter().collect()
	)
}

use rstd::prelude::*;
use rstd::ops::Deref;

#[cfg(feature = "std")]
pub mod bytes;
#[cfg(feature = "std")]
pub mod hashing;
#[cfg(feature = "std")]
pub use hashing::{blake2_256, twox_128, twox_256};
#[cfg(feature = "std")]
pub mod hexdisplay;

pub mod hash;
pub mod sandbox;
pub mod storage;
pub mod uint;
mod authority_id;

#[cfg(test)]
mod tests;

pub use self::hash::{H160, H256, H512};
pub use self::uint::{U256, U512};
pub use authority_id::AuthorityId;

/// A 512-bit value interpreted as a signature.
pub type Signature = hash::H512;

/// Hex-serialised shim for `Vec<u8>`.
#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug, Hash, PartialOrd, Ord))]
pub struct Bytes(#[cfg_attr(feature = "std", serde(with="bytes"))] pub Vec<u8>);

impl From<Vec<u8>> for Bytes {
	fn from(s: Vec<u8>) -> Self { Bytes(s) }
}

impl Deref for Bytes {
	type Target = [u8];
	fn deref(&self) -> &[u8] { &self.0[..] }
}
