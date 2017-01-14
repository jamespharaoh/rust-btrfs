use diskformat::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone) ]
pub struct BtrfsDevItem {
	device_id: u64,
	num_bytes: u64,
	num_bytes_used: u64,
	optimal_io_align: u32,
	optimal_io_width: u32,
	minimal_io_size: u32,
	device_type: u64,
	generation: u64,
	start_offset: u64,
	dev_group: u32,
	seek_speed: u8,
	bandwidth: u8,
	device_uuid: BtrfsUuid,
	fs_uuid: BtrfsUuid,
}

impl BtrfsDevItem {

	pub fn device_id (& self) -> u64 {
		self.device_id
	}

}

// ex: noet ts=4 filetype=rust
