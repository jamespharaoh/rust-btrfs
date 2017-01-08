use std::mem;

use diskformat::*;

#[ derive (Clone, Debug) ]
pub enum BtrfsItem <'a> {
	InodeItem (BtrfsInodeItem <'a>),
	InternalItem (BtrfsInternalItem <'a>),
	DirItem (BtrfsDirItem <'a>),
	DirIndex (BtrfsDirIndex <'a>),
	ExtentData (BtrfsExtentData <'a>),
	ExtentItem (BtrfsExtentItem <'a>),
	Unknown (BtrfsUnknownItem <'a>),
	Invalid (BtrfsInvalidItem <'a>),
}

#[ derive (Clone, Debug) ]
pub struct BtrfsUnknownItem <'a> {
	header: & 'a BtrfsItemHeader,
	data_bytes: & 'a [u8],
}

#[ derive (Clone, Debug) ]
pub struct BtrfsInvalidItem <'a> {
	header: & 'a BtrfsItemHeader,
	data_bytes: & 'a [u8],
	error: String,
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInternalItem <'a> {
	key: & 'a BtrfsKey,
	block_number: u64,
	generation: u64,
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsItemHeader {
	key: BtrfsKey,
	data_offset: u32,
	data_size: u32,
}

impl <'a> BtrfsItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsItemHeader,
		data_bytes: & 'a [u8],
	) -> BtrfsItem <'a> {

		match header.key.item_type () {

			BTRFS_INODE_ITEM_TYPE =>
				BtrfsInodeItem::from_bytes (
					header,
					data_bytes,
				).map (
					|inode_item|

					BtrfsItem::InodeItem (
						inode_item)

				),

			BTRFS_DIR_ITEM_TYPE =>
				BtrfsDirItem::from_bytes (
					header,
					data_bytes,
				).map (
					|dir_item|

					BtrfsItem::DirItem (
						dir_item)

				),

			BTRFS_DIR_INDEX_TYPE =>
				BtrfsDirIndex::from_bytes (
					header,
					data_bytes,
				).map (
					|dir_index|

					BtrfsItem::DirIndex (
						dir_index)

				),

			BTRFS_EXTENT_DATA_TYPE =>
				BtrfsExtentData::from_bytes (
					header,
					data_bytes,
				).map (
					|extent_data|

					BtrfsItem::ExtentData (
						extent_data)

				),

			BTRFS_EXTENT_ITEM_TYPE =>
				BtrfsExtentItem::from_bytes (
					header,
					data_bytes,
				).map (
					|extent_item|

					BtrfsItem::ExtentItem (
						extent_item)

				),

			_ =>
				Ok (
					BtrfsItem::Unknown (
						BtrfsUnknownItem {
							header: header,
							data_bytes: data_bytes,
						}
					)
				),

		}.unwrap_or_else (
			|error|

			BtrfsItem::Invalid (
				BtrfsInvalidItem {
					header: header,
					data_bytes: data_bytes,
					error: error,
				}
			)

		)

	}

}

impl BtrfsItemHeader {

	pub fn from_bytes (
		bytes: & [u8],
	) -> Result <& BtrfsItemHeader, String> {

		// sanity check

		if bytes.len () != mem::size_of::<BtrfsItemHeader> () {

			return Err (
				format! (
					"Must be exactly 0x{:x} bytes",
					mem::size_of::<BtrfsItemHeader> ()));

		}

		// cast

		Ok (
			unsafe {
				& * (bytes.as_ptr () as * const BtrfsItemHeader)
			}
		)

	}

	pub fn key (& self) -> BtrfsKey {
		self.key
	}

	pub fn object_id (& self) -> u64 {
		self.key.object_id ()
	}

	pub fn item_type (& self) -> u8 {
		self.key.item_type ()
	}

	pub fn offset (& self) -> u64 {
		self.key.offset ()
	}

	pub fn data_offset (& self) -> u32 {
		self.data_offset
	}

	pub fn data_size (& self) -> u32 {
		self.data_size
	}

}

/*
impl From <u8> for BtrfsItemType {

	pub fn from (
		value: u8,
	) -> BtrfsItemType {

		match value {

			BTRFS_INODE_ITEM_TYPE: BtrfsItemType::InodeItem,
			BTRFS_DIR_ITEM_TYPE: BtrfsItemType::DirItem,
			BTRFS_DIR_INDEX_TYPE: BtrfsItemType::DirIndex,
			BTRFS_EXTENT_DATA_TYPE: BtrfsItemType::ExtentData,
			BTRFS_EXTENT_DATA_ITEM: BtrfsItemType::ExtentItem,

			_ => panic! (
				"Unrecognised BTRFS item type 0x{2:x}",
				value),

		}

	}

}
*/

pub const BTRFS_INODE_ITEM_TYPE: u8 = 0x01;
pub const BTRFS_DIR_ITEM_TYPE: u8 = 0x54;
pub const BTRFS_DIR_INDEX_TYPE: u8 = 0x60;
pub const BTRFS_EXTENT_DATA_TYPE: u8 = 0x6c;
pub const BTRFS_EXTENT_ITEM_TYPE: u8 = 0xa8;

// ex: noet ts=4 filetype=rust
