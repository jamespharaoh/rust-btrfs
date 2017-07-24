#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsExtentDataData {

	pub generation: u64,
	pub logical_data_size: u64,
	pub compression: u8,
	pub encryption: u8,
	pub other_encoding: u16,
	pub extent_type: u8,

	pub extent_logical_address: u64,
	pub extent_size: u64,
	pub extent_data_offset: u64,
	pub extent_data_size: u64,

}

#[ cfg (test) ]
mod tests {

	use std::mem;

	use super::*;

	#[ test ]
	fn test_size () {
		assert! (mem::size_of::<BtrfsExtentDataData> () == 0x35);
	}

}

// ex: noet ts=4 filetype=rust
