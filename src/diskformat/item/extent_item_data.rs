#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsExtentItemData {
	pub reference_count: u64,
	pub generation: u64,
	pub flags: u64,
}

#[ cfg (test) ]
mod tests {

	use std::mem;

	use super::*;

	#[ test ]
	fn test_size () {
		assert! (mem::size_of::<BtrfsExtentItemData> () == 0x18);
	}

}

// ex: noet ts=4 filetype=rust
