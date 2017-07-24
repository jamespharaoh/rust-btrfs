#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsExtentDataRefData {
	tree_id: u64,
	object_id: u64,
	offset: u64,
	reference_count: u64,
}

// ex: noet ts=4 filetype=rust
