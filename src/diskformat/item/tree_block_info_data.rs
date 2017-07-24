use super::super::prelude::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsTreeBlockInfoData {
	pub first_entry_key: BtrfsKey,
	pub level: u8,
}

// ex: noet ts=4 filetype=rust
