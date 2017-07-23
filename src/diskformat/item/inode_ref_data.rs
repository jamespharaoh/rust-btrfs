#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInodeRefData {
	pub sequence: u64,
	pub name_length: u16,
}

// ex: noet ts=4 filetype=rust
