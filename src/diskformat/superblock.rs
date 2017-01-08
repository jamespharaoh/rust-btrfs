use diskformat::*;

#[ repr (C, packed) ]
#[ derive (Copy) ]
pub struct BtrfsSuperblock {
	pub checksum: BtrfsChecksum,
	pub fs_uuid: BtrfsUuid,
	pub physical_address: u64,
	pub flags: u64,
	pub magic: [u8; 0x8],
	pub generation: u64,
	pub root_tree_logical_address: u64,
	pub chunk_tree_logical_address: u64,
	pub log_tree_logical_address: u64,
	pub log_root_transid: u64,
	pub total_bytes: u64,
	pub bytes_used: u64,
	pub root_dir_objectid: u64,
	pub num_devices: u64,
	pub sector_size: u32,
	pub node_size: u32,
	pub leaf_size: u32,
	pub stripesize: u32,
	pub n: u32,
	pub chunk_root_generation: u64,
	pub compat_flags: u64,
	pub compat_ro_flags: u64,
	pub incompat_flags: u64,
	pub csum_type: u16,
	pub root_level: u8,
	pub chunk_root_level: u8,
	pub log_root_level: u8,
	pub dev_item: BtrfsDevItem,
	pub label: BtrfsLabel,
	pub reserved: [u8; 0x100],
	pub system_chunks: [u8; 0x800],
	pub unused: [u8; 0x4d5],
}

impl BtrfsSuperblock {

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.fs_uuid
	}

}

impl Clone for BtrfsSuperblock {

	fn clone (
		& self,
	) -> Self {
		* self
	}

}

// ex: noet ts=4 filetype=rust
