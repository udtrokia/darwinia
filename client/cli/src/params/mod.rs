mod import_params;
mod network_configuration_params;
mod node_key_params;
mod pruning_params;
mod shared_params;
mod transaction_pool_params;

use std::fmt::Debug;
use std::str::FromStr;

pub use crate::params::import_params::*;
pub use crate::params::network_configuration_params::*;
pub use crate::params::node_key_params::*;
pub use crate::params::pruning_params::*;
pub use crate::params::shared_params::*;
pub use crate::params::transaction_pool_params::*;

/// Wrapper type of `String` that holds an unsigned integer of arbitrary size, formatted as a decimal.
#[derive(Debug, Clone)]
pub struct BlockNumber(String);

impl FromStr for BlockNumber {
	type Err = String;

	fn from_str(block_number: &str) -> Result<Self, Self::Err> {
		if block_number.chars().any(|d| !d.is_digit(10)) {
			Err(format!(
				"Invalid block number: {}, expected decimal formatted unsigned integer",
				block_number,
			))
		} else {
			Ok(Self(block_number.to_owned()))
		}
	}
}

impl BlockNumber {
	/// Wrapper on top of `std::str::parse<N>` but with `Error` as a `String`
	///
	/// See `https://doc.rust-lang.org/std/primitive.str.html#method.parse` for more elaborate
	/// documentation.
	pub fn parse<N>(&self) -> Result<N, String>
	where
		N: FromStr,
		N::Err: std::fmt::Debug,
	{
		self.0
			.parse()
			.map_err(|e| format!("BlockNumber: {} parsing failed because of {:?}", self.0, e))
	}
}
