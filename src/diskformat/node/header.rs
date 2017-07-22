use std::fmt::Debug;
use std::fmt::DebugStruct;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, PartialEq) ]
pub struct BtrfsNodeHeader {
	pub checksum: BtrfsChecksum,
	pub fs_uuid: BtrfsUuid,
	pub logical_address: u64,
	pub flags_and_backref: u64,
	pub chunk_tree_uuid: BtrfsUuid,
	pub generation: u64,
	pub tree_id: BtrfsTreeId,
	pub num_items: u32,
	pub level: u8,
}

impl BtrfsNodeHeader {

	pub fn checksum (& self) -> BtrfsChecksum {
		self.checksum
	}

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.fs_uuid
	}

	pub fn generation (& self) -> u64 {
		self.generation
	}

	pub fn tree_id (& self) -> BtrfsTreeId {
		self.tree_id
	}

	pub fn num_items (& self) -> u32 {
		self.num_items
	}

	pub fn level (& self) -> u8 {
		self.level
	}

	pub fn debug_struct (
		& self,
		debug_struct: & mut DebugStruct,
	) {

		debug_struct.field (
			"checksum",
			& NakedString::from (
				self.checksum.to_string ()));

		debug_struct.field (
			"fs_uuid",
			& NakedString::from (
				self.fs_uuid.to_string ()));

		debug_struct.field (
			"generation",
			& self.generation);

		debug_struct.field (
			"tree_id",
			& NakedString::from (
				self.tree_id.to_string ()));

		debug_struct.field (
			"num_items",
			& self.num_items);

		debug_struct.field (
			"level",
			& self.level);

	}

}

impl Debug for BtrfsNodeHeader {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsNodeHeader");

		self.debug_struct (
			& mut debug_struct);

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
