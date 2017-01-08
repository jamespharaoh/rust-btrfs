#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlFileDedupeRangeInfo {
	pub dest_fd: i64,
	pub dest_offset: u64,
	pub bytes_deduped: u64,
	pub status: i32,
	pub reserved: u32,
}

// ex: noet ts=4 filetype=rust
