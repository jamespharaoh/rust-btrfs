use diskformat::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, PartialEq) ]
pub struct BtrfsNodeHeader {
	checksum: BtrfsChecksum,
	fs_uuid: BtrfsUuid,
	logical_address: u64,
	flags_and_backref: u64,
	chunk_tree_uuid: BtrfsUuid,
	generation: u64,
	tree_id: u64,
	num_items: u32,
	level: u8,
}

impl BtrfsNodeHeader {

	pub fn checksum (& self) -> BtrfsChecksum {
		self.checksum
	}

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.fs_uuid
	}

	pub fn tree_id (& self) -> u64 {
		self.tree_id
	}

	pub fn num_items (& self) -> u32 {
		self.num_items
	}

	pub fn level (& self) -> u8 {
		self.level
	}

}

// ex: noet ts=4 filetype=rust
