use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsDirIndexData {
	pub child_key: BtrfsKey,
	pub transaction_id: u64,
	pub data_size: u16,
	pub name_size: u16,
	pub child_type: u8,
}

// ex: noet ts=4 filetype=rust
