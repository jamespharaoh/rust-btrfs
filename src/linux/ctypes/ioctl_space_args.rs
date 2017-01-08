#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlSpaceArgs {
	pub space_slots: u64,
	pub total_spaces: u64,
}

// ex: noet ts=4 filetype=rust
