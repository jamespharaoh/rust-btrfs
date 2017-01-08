#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlFiemapExtent {
	pub logical: u64,
	pub physical: u64,
	pub length: u64,
	pub reserved0: u64,
	pub reserved1: u64,
	pub flags: u32,
	pub reserved2: u32,
	pub reserved3: u32,
	pub reserved4: u32,
}

// ex: noet ts=4 filetype=rust
