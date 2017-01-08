#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlDefragRangeArgs {
	pub start: u64,
	pub len: u64,
	pub flags: u64,
	pub extent_thresh: u32,
	pub compress_type: u32,
	pub unused_0: u32,
	pub unused_1: u32,
	pub unused_2: u32,
	pub unused_3: u32,
}

// ex: noet ts=4 filetype=rust
