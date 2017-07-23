#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsRootRefData {
	pub directory_id: u64,
	pub sequence: u64,
	pub name_length: u16,
}

#[ cfg (test) ]
mod tests {

	use std::mem;

	use super::*;

	#[ test ]
	fn test_size () {
		assert! (mem::size_of::<BtrfsRootRefData> () == 0x12);
	}

}

// ex: noet ts=4 filetype=rust
