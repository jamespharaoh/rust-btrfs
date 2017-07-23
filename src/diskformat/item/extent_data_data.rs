#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsExtentDataData {
	pub generation: u64,
	pub logical_data_size: u64,
	pub compression: u8,
	pub encryption: u8,
	pub other_encoding: u16,
	pub extent_type: u8,
	pub logical_address: u64,
	pub extent_size: u64,
	pub extent_offset: u64,
	pub logical_bytes: u64,
}

// ex: noet ts=4 filetype=rust
