use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use itertools::Itertools;

pub const BTRFS_UUID_BYTES: usize = 0x10;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsUuid {
	bytes: [u8; BTRFS_UUID_BYTES],
}

impl BtrfsUuid {

	pub fn bytes (& self) -> & [u8] {
		& self.bytes
	}

	pub fn to_string (& self) -> String {

		format! (
			"{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
			self.bytes [0x00 .. 0x04].iter ().format (""),
			self.bytes [0x04 .. 0x06].iter ().format (""),
			self.bytes [0x06 .. 0x08].iter ().format (""),
			self.bytes [0x08 .. 0x0a].iter ().format (""),
			self.bytes [0x0a .. 0x10].iter ().format (""))

	}

}

impl Debug for BtrfsUuid {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"BtrfsUuid ({:02x}-{:02x}-{:02x}-{:02x}-{:02x})",
				self.bytes [0x00 .. 0x04].iter ().format (""),
				self.bytes [0x04 .. 0x06].iter ().format (""),
				self.bytes [0x06 .. 0x08].iter ().format (""),
				self.bytes [0x08 .. 0x0a].iter ().format (""),
				self.bytes [0x0a .. 0x10].iter ().format ("")))

	}

}

impl Display for BtrfsUuid {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
				self.bytes [0x00 .. 0x04].iter ().format (""),
				self.bytes [0x04 .. 0x06].iter ().format (""),
				self.bytes [0x06 .. 0x08].iter ().format (""),
				self.bytes [0x08 .. 0x0a].iter ().format (""),
				self.bytes [0x0a .. 0x10].iter ().format ("")))

	}

}

// ex: noet ts=4 filetype=rust
