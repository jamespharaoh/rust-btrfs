use std::collections::HashMap;
use std::slice;

use crc::crc32;

#[ derive (Clone) ]
pub struct BtrfsDevice {
	pointer: * const u8,
	len: usize,
}

impl BtrfsDevice {

	pub fn new (
		pointer: * const u8,
		len: usize,
	) -> BtrfsDevice {

		BtrfsDevice {
			pointer: pointer,
			len: len,
		}

	}

	pub fn slice_at (
		& self,
		offset: usize,
		len: usize,
	) -> & [u8] {

		if offset >= self.len {

			panic! (
				"Device slice start out of range: 0x{:x}",
				offset);

		}

		if offset + len >= self.len {

			panic! (
				"Device slice end out of range: 0x{:x}",
				offset + len);

		}

		unsafe {
			slice::from_raw_parts (
				self.pointer.offset (
					offset as isize),
				len,
			)
		}

	}

	pub fn pointer (& self) -> * const u8 {
		self.pointer
	}

	pub fn len (& self) -> usize {
		self.len
	}

}

pub type BtrfsDeviceMap = HashMap <u64, BtrfsDevice>;

pub type BtrfsUuid = [u8; BTRFS_UUID_BYTES];

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsChecksum {
	bytes: [u8; BTRFS_CHECKSUM_BYTES],
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsKey {
	object_id: u64,
	item_type: u8,
	offset: u64,
}

#[ repr (C, packed) ]
#[ derive (Copy) ]
pub struct BtrfsLabel {
	value: [u8; BTRFS_LABEL_BYTES],
}

impl BtrfsChecksum {

	pub fn for_bytes (
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

}

impl BtrfsKey {

	pub fn object_id (& self) -> u64 {
		self.object_id
	}

	pub fn item_type (& self) -> u8 {
		self.item_type
	}

	pub fn offset (& self) -> u64 {
		self.offset
	}

}

impl Clone for BtrfsLabel {

	fn clone (
		& self,
	) -> Self {
		* self
	}

}

pub const BTRFS_MAGIC: [u8; 0x8] = * b"_BHRfS_M";

pub const BTRFS_CHECKSUM_BYTES: usize = 0x20;
pub const BTRFS_UUID_BYTES: usize = 0x10;
pub const BTRFS_LABEL_BYTES: usize = 0x100;

pub const BTRFS_CHILD_UNKNOWN_TYPE: u8 = 0;
pub const BTRFS_CHILD_REGULAR_FILE_TYPE: u8 = 1;
pub const BTRFS_CHILD_DIRECTORY_TYPE: u8 = 2;
pub const BTRFS_CHILD_CHARACTER_DEVICE_TYPE: u8 = 3;
pub const BTRFS_CHILD_BLOCK_DEVICE_TYPE: u8 = 4;
pub const BTRFS_CHILD_FIFO_TYPE: u8 = 5;
pub const BTRFS_CHILD_SOCKET_TYPE: u8 = 6;
pub const BTRFS_CHILD_SYMBOLIC_LINK_TYPE: u8 = 7;
pub const BTRFS_CHILD_EXTENDED_ATTRIBUTE_TYPE: u8 = 8;

// ex: noet ts=4 filetype=rust
