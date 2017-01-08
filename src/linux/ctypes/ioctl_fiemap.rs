#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlFiemap {
	pub start: u64,
	pub length: u64,
	pub flags: u32,
	pub mapped_extents: u32,
	pub extent_count: u32,
	pub reserved: u32,
}

// ex: noet ts=4 filetype=rust
