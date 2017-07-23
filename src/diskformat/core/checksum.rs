use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use crc::crc32;

use itertools::Itertools;

pub const BTRFS_CHECKSUM_BYTES: usize = 0x20;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsChecksum {
	bytes: [u8; BTRFS_CHECKSUM_BYTES],
}

pub fn btrfs_crc32c (
	bytes: & [u8],
) -> u32 {

	crc32::checksum_castagnoli (
		bytes)

}

pub fn btrfs_crc32_linux (
	bytes: & [u8],
) -> u32 {

	! crc32::update (
		1,
		& crc32::CASTAGNOLI_TABLE,
		bytes)

}

impl BtrfsChecksum {

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

impl From <u32> for BtrfsChecksum {

	fn from (
		value: u32,
	) -> BtrfsChecksum {

		BtrfsChecksum {
			bytes: [
				((value & 0x000000ff) >> 0) as u8,
				((value & 0x0000ff00) >> 8) as u8,
				((value & 0x00ff0000) >> 16) as u8,
				((value & 0xff000000) >> 24) as u8,
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

}

// ex: noet ts=4 filetype=rust
