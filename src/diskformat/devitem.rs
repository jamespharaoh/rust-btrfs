use diskformat::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone) ]
pub struct BtrfsDevItem {
	pub device_id: u64,
	pub num_bytes: u64,
	pub num_bytes_used: u64,
	pub optimal_io_align: u32,
	pub optimal_io_width: u32,
	pub minimal_io_size: u32,
	pub device_type: u64,
	pub generation: u64,
	pub start_offset: u64,
	pub dev_group: u32,
	pub seek_speed: u8,
	pub bandwidth: u8,
	pub device_uuid: BtrfsUuid,
	pub fs_uuid: BtrfsUuid,
}

// ex: noet ts=4 filetype=rust
