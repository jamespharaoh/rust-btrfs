#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlSpaceInfo {
	pub flags: u64,
	pub total_bytes: u64,
	pub used_bytes: u64,
}

// ex: noet ts=4 filetype=rust
