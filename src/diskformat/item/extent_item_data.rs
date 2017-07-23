use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsExtentItemData {
	pub reference_count: u64,
	pub generation: u64,
	pub flags: u64,
	pub first_entry_key: BtrfsKey,
	pub level: u8,
}

// ex: noet ts=4 filetype=rust
