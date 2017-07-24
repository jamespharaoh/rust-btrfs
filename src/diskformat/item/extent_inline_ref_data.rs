#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsInlineRefData {
	inline_ref_type: u8,
	offset: u64,
}

// ex: noet ts=4 filetype=rust
