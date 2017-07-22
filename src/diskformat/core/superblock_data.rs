use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Eq, Hash, PartialEq) ]
pub struct BtrfsSuperblockData {
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
	pub stripe_size: u32,
	pub system_chunks_size: u32,
	pub chunk_root_generation: u64,
	pub compat_flags: u64,
	pub compat_ro_flags: u64,
	pub incompat_flags: u64,
	pub csum_type: u16,
	pub root_level: u8,
	pub chunk_root_level: u8,
	pub log_root_level: u8,
	pub dev_item: BtrfsDevItemData,
	pub label: BtrfsLabel,
	pub cache_generation: u64,
	pub uuid_tree_generation: u64,
	pub reserved: BtrfsSuperblockReserved,
	pub system_chunks: BtrfsSuperblockSystemChunksData,
	pub root_backups: [BtrfsRootBackup; 4],
	pub unused: BtrfsSuperblockUnused,
}

impl BtrfsSuperblockData {

	pub fn for_bytes (
		bytes: & [u8],
	) -> & BtrfsSuperblockData {

		assert! (
			bytes.len () == mem::size_of::<BtrfsSuperblockData> ());

		let superblock_data: & BtrfsSuperblockData =
			unsafe {
				mem::transmute (
					& bytes [0])
			};

		// TODO verify stuff

		superblock_data

	}

}

impl Clone for BtrfsSuperblockData {

	fn clone (& self) -> BtrfsSuperblockData {
		* self
	}

}

impl Debug for BtrfsSuperblockData {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsSuperblockData");

		debug_struct.field (
			"checksum",
			& self.checksum);

		debug_struct.field (
			"fs_uuid",
			& self.fs_uuid);

		debug_struct.field (
			"physical_address",
			& self.physical_address);

		debug_struct.field (
			"flags",
			& self.flags);

		debug_struct.field (
			"magic",
			& self.magic);

		debug_struct.field (
			"generation",
			& self.generation);

		debug_struct.field (
			"root_tree_logical_address",
			& self.root_tree_logical_address);

		debug_struct.field (
			"chunk_tree_logical_address",
			& self.chunk_tree_logical_address);

		debug_struct.field (
			"log_tree_logical_address",
			& self.log_tree_logical_address);

		debug_struct.field (
			"log_root_transid",
			& self.log_root_transid);

		debug_struct.field (
			"total_bytes",
			& self.total_bytes);

		debug_struct.field (
			"bytes_used",
			& self.bytes_used);

		debug_struct.field (
			"root_dir_objectid",
			& self.root_dir_objectid);

		debug_struct.field (
			"num_devices",
			& self.num_devices);

		debug_struct.field (
			"sector_size",
			& self.sector_size);

		debug_struct.field (
			"node_size",
			& self.node_size);

		debug_struct.field (
			"leaf_size",
			& self.leaf_size);

		debug_struct.field (
			"stipe_size",
			& self.stripe_size);

		debug_struct.field (
			"system_chunks_size",
			& self.system_chunks_size);

		debug_struct.field (
			"chunk_root_generation",
			& self.chunk_root_generation);

		debug_struct.field (
			"compat_flags",
			& self.compat_flags);

		debug_struct.field (
			"compat_ro_flags",
			& self.compat_ro_flags);

		debug_struct.field (
			"incompat_flags",
			& self.incompat_flags);

		debug_struct.field (
			"csum_type",
			& self.csum_type);

		debug_struct.field (
			"root_level",
			& self.root_level);

		debug_struct.field (
			"chunk_root_level",
			& self.chunk_root_level);

		debug_struct.field (
			"log_root_level",
			& self.log_root_level);

		debug_struct.field (
			"dev_item",
			& self.dev_item);

		debug_struct.field (
			"label",
			& self.label);

		debug_struct.field (
			"reserved",
			& "TODO".to_string ());

		debug_struct.field (
			"system_chunks",
			& "TODO".to_string ());

		debug_struct.field (
			"unused",
			& "...".to_string ());

		debug_struct.finish () ?;

		Ok (())

	}

}

#[ test ]
fn test_size () {
	assert! (mem::size_of::<BtrfsSuperblockData> () == 0x1000);
}

// ex: noet ts=4 filetype=rust
