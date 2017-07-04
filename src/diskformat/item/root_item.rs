use std::fmt;
use std::mem;

use diskformat::*;

#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsRootItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsRootItemData {
	inode_item: BtrfsInodeItemData,
	expected_generation: u64,
	root_object_id: u64,
	root_node_block_number: u64,
	byte_limit: u64,
	bytes_used: u64,
	last_snapshot_generation: u64,
	flags: u64,
	num_references: u32,
	drop_progress: BtrfsKey,
	drop_level: u8,
	tree_level: u8,
	generation_v2: u64,
	subvolume_uuid: BtrfsUuid,
	parent_uuid: BtrfsUuid,
	received_uuid: BtrfsUuid,
	changed_transaction_id: u64,
	created_transaction_id: u64,
	sent_transaction_id: u64,
	received_transaction_id: u64,
	changed_time: [u8; 0xc],
	created_time: [u8; 0xc],
	sent_time: [u8; 0xc],
	received_time: [u8; 0xc],
	reserved: [u64; 0x8],
}

impl <'a> BtrfsRootItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsRootItem <'a>, String> {

		// sanity check

		if data_bytes.len () != mem::size_of::<BtrfsRootItemData> () {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsRootItemData> ()));

		}

		// create root item

		let root_item = BtrfsRootItem {
			header: header,
			data_bytes: data_bytes,
		};

		// return

		Ok (root_item)

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

	pub fn key (& self) -> BtrfsKey {
		self.header.key ()
	}

	pub fn object_id (& self) -> u64 {
		self.header.object_id ()
	}

	pub fn data (& self) -> & BtrfsRootItemData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsRootItemData
			)
		}

	}

	pub fn inode_item (& self) -> & BtrfsInodeItemData {
		& self.data ().inode_item
	}

	pub fn expected_generation (& self) -> u64 {
		self.data ().expected_generation
	}

	pub fn root_object_id (& self) -> u64 {
		self.data ().root_object_id
	}

	pub fn root_node_block_number (& self) -> u64 {
		self.data ().root_node_block_number
	}

	pub fn byte_limit (& self) -> u64 {
		self.data ().byte_limit
	}

	pub fn bytes_used (& self) -> u64 {
		self.data ().bytes_used
	}

	pub fn last_snapshot_generation (& self) -> u64 {
		self.data ().last_snapshot_generation
	}

	pub fn flags (& self) -> u64 {
		self.data ().flags
	}

	pub fn num_references (& self) -> u32 {
		self.data ().num_references
	}

	pub fn drop_progress (& self) -> BtrfsKey {
		self.data ().drop_progress
	}

	pub fn drop_level (& self) -> u8 {
		self.data ().drop_level
	}

	pub fn tree_level (& self) -> u8 {
		self.data ().tree_level
	}

}

impl <'a> fmt::Debug for BtrfsRootItem <'a> {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> Result <(), fmt::Error> {

		write! (
			formatter,
			"BtrfsRootItem: {{ header: ",
		) ?;

		self.header.fmt (
			formatter,
		) ?;

		write! (
			formatter,
			", ",
		) ?;

		self.data ().fmt (
			formatter,
		) ?;

		write! (
			formatter,
			" }}",
		) ?;

		Ok (())

	}

}

// ex: noet ts=4 filetype=rust
