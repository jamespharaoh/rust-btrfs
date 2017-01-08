use linux::imports::*;

#[ derive (Debug, Eq, PartialEq) ]
pub struct FilesystemInfo {
	pub max_id: u64,
	pub num_devices: u64,
	pub filesystem_id: Uuid,
}

// ex: noet ts=4 filetype=rust
