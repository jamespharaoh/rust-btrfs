#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlFileDedupeRange {
	pub src_offset: u64,
	pub src_length: u64,
	pub dest_count: u16,
	pub reserved1: u16,
	pub reserved2: u16,
}

// ex: noet ts=4 filetype=rust
