use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use chrono::NaiveDateTime;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsTimestamp {
	seconds: i64,
	nano_seconds: u32,
}

impl BtrfsTimestamp {

	pub fn new (
		seconds: i64,
		nano_seconds: u32,
	) -> BtrfsTimestamp {

		BtrfsTimestamp {
			seconds: seconds,
			nano_seconds: nano_seconds,
		}

	}

	pub fn seconds (& self) -> i64 {
		self.seconds
	}

	pub fn nano_seconds (& self) -> u32 {
		self.nano_seconds
	}

	pub fn to_naive_date_time (& self) -> NaiveDateTime {

		NaiveDateTime::from_timestamp (
			self.seconds,
			self.nano_seconds)

	}

	pub fn to_string (& self) -> String {

		self.to_naive_date_time ().format (
			"%Y-%m-%dT%H:%M:%S%.fZ",
		).to_string ()

	}

}

impl Debug for BtrfsTimestamp {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"BtrfsTimestamp ({})",
				self.to_naive_date_time ().format (
					"%Y-%m-%dT%H:%M:%S%.fZ",
				)))

	}

}

impl Display for BtrfsTimestamp {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"{}",
				self.to_naive_date_time ().format (
					"%Y-%m-%dT%H:%M:%S%.fZ",
				)))

	}

}

// ex: noet ts=4 filetype=rust
