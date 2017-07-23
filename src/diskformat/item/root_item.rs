use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use super::super::*;

#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsRootItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
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

impl <'a> BtrfsLeafItemContents <'a> for BtrfsRootItem <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

impl <'a> Debug for BtrfsRootItem <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsRootItem");

		self.header.debug_struct (
			& mut debug_struct);

		self.data ().debug_struct (
			& mut debug_struct);

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
