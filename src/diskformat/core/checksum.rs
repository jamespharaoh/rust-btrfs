use std::fmt::Arguments as FmtArguments;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use crc::crc32;

use itertools::Itertools;

use super::super::*;

pub const BTRFS_CHECKSUM_BYTES: usize = 0x20;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsChecksum {
	bytes: [u8; BTRFS_CHECKSUM_BYTES],
}

impl BtrfsChecksum {

	pub fn calculate_crc32 (
		bytes: & [u8],
	) -> BtrfsChecksum {

		let checksum_u32 =
			crc32::checksum_castagnoli (
				bytes);

		BtrfsChecksum {
			bytes: [
				((checksum_u32 & 0x000000ff) >> 0) as u8,
				((checksum_u32 & 0x0000ff00) >> 8) as u8,
				((checksum_u32 & 0x00ff0000) >> 16) as u8,
				((checksum_u32 & 0xff000000) >> 24) as u8,
				0, 0, 0, 0,
				0, 0, 0, 0,
				0, 0, 0, 0,
				0, 0, 0, 0,
				0, 0, 0, 0,
				0, 0, 0, 0,
				0, 0, 0, 0,
			]
		}

	}

	pub fn bytes (& self) -> & [u8] {
		& self.bytes
	}

	pub fn bytes_truncated (& self) -> & [u8] {

		if self.bytes.iter ().skip (4).all (
			|byte| * byte == 0
		) {

			& self.bytes [0 .. 4]

		} else {

			& self.bytes

		}

	}

	pub fn to_string (
		& self,
	) -> String {

		format! (
			"{:02x}",
			self.bytes_truncated ().iter ().format (""))

	}

}

impl Debug for BtrfsChecksum {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"BtrfsChecksum ({})",
				self.to_string ()))

	}

}

impl Display for BtrfsChecksum {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"{}",
				self.to_string ()))

	}

}

// ex: noet ts=4 filetype=rust
