use std::mem;

use diskformat::*;

#[ derive (Clone, Debug) ]
pub enum BtrfsLeafItem <'a> {
	ChunkItem (BtrfsChunkItem <'a>),
	DirIndex (BtrfsDirIndex <'a>),
	DirItem (BtrfsDirItem <'a>),
	ExtentData (BtrfsExtentData <'a>),
	ExtentItem (BtrfsExtentItem <'a>),
	InodeItem (BtrfsInodeItem <'a>),
	Invalid (BtrfsInvalidItem <'a>),
	Unknown (BtrfsUnknownItem <'a>),
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsLeafItemHeader {
	key: BtrfsKey,
	data_offset: u32,
	data_size: u32,
}

impl <'a> BtrfsLeafItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> BtrfsLeafItem <'a> {

		match header.key.item_type () {

			BTRFS_INODE_ITEM_TYPE =>
				BtrfsInodeItem::from_bytes (
					header,
					data_bytes,
				).map (
					|inode_item|

					BtrfsLeafItem::InodeItem (
						inode_item)

				),

			BTRFS_DIR_ITEM_TYPE =>
				BtrfsDirItem::from_bytes (
					header,
					data_bytes,
				).map (
					|dir_item|

					BtrfsLeafItem::DirItem (
						dir_item)

				),

			BTRFS_DIR_INDEX_TYPE =>
				BtrfsDirIndex::from_bytes (
					header,
					data_bytes,
				).map (
					|dir_index|

					BtrfsLeafItem::DirIndex (
						dir_index)

				),

			BTRFS_EXTENT_DATA_TYPE =>
				BtrfsExtentData::from_bytes (
					header,
					data_bytes,
				).map (
					|extent_data|

					BtrfsLeafItem::ExtentData (
						extent_data)

				),

			BTRFS_EXTENT_ITEM_TYPE =>
				BtrfsExtentItem::from_bytes (
					header,
					data_bytes,
				).map (
					|extent_item|

					BtrfsLeafItem::ExtentItem (
						extent_item)

				),

			BTRFS_CHUNK_ITEM_TYPE =>
				BtrfsChunkItem::from_bytes (
					header,
					data_bytes,
				).map (
					|chunk_item|

					BtrfsLeafItem::ChunkItem (
						chunk_item)

				),

			_ =>
				Ok (
					BtrfsLeafItem::Unknown (
						BtrfsUnknownItem::new (
							header,
							data_bytes,
						)
					)
				),

		}.unwrap_or_else (
			|error|

			BtrfsLeafItem::Invalid (
				BtrfsInvalidItem::new (
					header,
					data_bytes,
					error,
				)
			)

		)

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {

		match self {

			& BtrfsLeafItem::ChunkItem (ref chunk_item) =>
				chunk_item.header (),

			& BtrfsLeafItem::DirIndex (ref dir_index) =>
				dir_index.header (),

			& BtrfsLeafItem::DirItem (ref dir_item) =>
				dir_item.header (),

			& BtrfsLeafItem::ExtentData (ref extent_data) =>
				extent_data.header (),

			& BtrfsLeafItem::ExtentItem (ref extent_item) =>
				extent_item.header (),

			& BtrfsLeafItem::InodeItem (ref inode_item) =>
				inode_item.header (),

			& BtrfsLeafItem::Unknown (ref unknown_item) =>
				unknown_item.header (),

			& BtrfsLeafItem::Invalid (ref invalid_item) =>
				invalid_item.header (),

		}

	}

	pub fn key (& self) -> BtrfsKey {
		self.header ().key
	}

}

impl BtrfsLeafItemHeader {

	pub fn from_bytes (
		bytes: & [u8],
	) -> Result <& BtrfsLeafItemHeader, String> {

		// sanity check

		if bytes.len () != mem::size_of::<BtrfsLeafItemHeader> () {

			return Err (
				format! (
					"Must be exactly 0x{:x} bytes",
					mem::size_of::<BtrfsLeafItemHeader> ()));

		}

		// cast

		Ok (
			unsafe {
				& * (bytes.as_ptr () as * const BtrfsLeafItemHeader)
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

pub const BTRFS_INODE_ITEM_TYPE: u8 = 0x01;
pub const BTRFS_DIR_ITEM_TYPE: u8 = 0x54;
pub const BTRFS_DIR_INDEX_TYPE: u8 = 0x60;
pub const BTRFS_EXTENT_DATA_TYPE: u8 = 0x6c;
pub const BTRFS_EXTENT_ITEM_TYPE: u8 = 0xa8;
pub const BTRFS_CHUNK_ITEM_TYPE: u8 = 0xe4;

// ex: noet ts=4 filetype=rust
