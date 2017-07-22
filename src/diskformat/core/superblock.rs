use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use super::super::*;

pub const BTRFS_MAGIC: [u8; 0x8] = * b"_BHRfS_M";

#[ derive (Copy, Clone, Eq, Hash, PartialEq) ]
pub struct BtrfsSuperblock <'a> {
	data: & 'a BtrfsSuperblockData,
}

impl <'a> BtrfsSuperblock <'a> {

	pub fn new (
		data: & 'a BtrfsSuperblockData,
	) -> BtrfsSuperblock <'a> {

		BtrfsSuperblock {
			data: data,
		}

	}

	pub fn for_bytes (
		bytes: & 'a [u8],
	) -> BtrfsSuperblock <'a> {

		Self::new (
			BtrfsSuperblockData::for_bytes (
				bytes))

	}

	pub fn system_chunks (
		self,
	) -> BtrfsSuperblockSystemChunks <'a> {

		BtrfsSuperblockSystemChunks::new (
			& self.data,
		)

	}

	pub fn system_logical_to_physical (
		& self,
		logical_address: u64,
	) -> Option <BtrfsPhysicalAddress> {

		for system_chunk in self.system_chunks () {

			if logical_address < system_chunk.key ().offset ()
			|| logical_address >= (
				system_chunk.key ().offset ()
				+ system_chunk.data ().chunk_size ()
			) {
				continue;
			}

			let system_chunk_stripe =
				system_chunk.stripes () [0];

			return Some (
				BtrfsPhysicalAddress::new (
					system_chunk_stripe.device_id (),
					logical_address
						- system_chunk.key ().offset ()
						+ system_chunk_stripe.offset ()));

		}

		None

	}

	pub fn checksum (& self) -> BtrfsChecksum {
		self.data.checksum
	}

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.data.fs_uuid
	}

	pub fn physical_address (& self) -> u64 {
		self.data.physical_address
	}

	pub fn flags (& self) -> u64 {
		self.data.flags
	}

	pub fn magic (& self) -> [u8; 0x8] {
		self.data.magic
	}

	pub fn generation (& self) -> u64 {
		self.data.generation
	}

	pub fn root_tree_logical_address (& self) -> u64 {
		self.data.root_tree_logical_address
	}

	pub fn chunk_tree_logical_address (& self) -> u64 {
		self.data.chunk_tree_logical_address
	}

	pub fn log_tree_logical_address (& self) -> u64 {
		self.data.log_tree_logical_address
	}

	pub fn log_root_transid (& self) -> u64 {
		self.data.log_root_transid
	}

	pub fn total_bytes (& self) -> u64 {
		self.data.total_bytes
	}

	pub fn bytes_used (& self) -> u64 {
		self.data.bytes_used
	}

	pub fn root_dir_objectid (& self) -> u64 {
		self.data.root_dir_objectid
	}

	pub fn num_devices (& self) -> u64 {
		self.data.num_devices
	}

	pub fn sector_size (& self) -> u32 {
		self.data.sector_size
	}

	pub fn node_size (& self) -> u32 {
		self.data.node_size
	}

	pub fn leaf_size (& self) -> u32 {
		self.data.leaf_size
	}

	pub fn stripe_size (& self) -> u32 {
		self.data.stripe_size
	}

	pub fn system_chunks_size (& self) -> u32 {
		self.data.system_chunks_size
	}

	pub fn chunk_root_generation (& self) -> u64 {
		self.data.chunk_root_generation
	}

	pub fn compat_flags (& self) -> u64 {
		self.data.compat_flags
	}

	pub fn compat_ro_flags (& self) -> u64 {
		self.data.compat_ro_flags
	}

	pub fn incompat_flags (& self) -> u64 {
		self.data.incompat_flags
	}

	pub fn csum_type (& self) -> u16 {
		self.data.csum_type
	}

	pub fn root_level (& self) -> u8 {
		self.data.root_level
	}

	pub fn chunk_root_level (& self) -> u8 {
		self.data.chunk_root_level
	}

	pub fn log_root_level (& self) -> u8 {
		self.data.log_root_level
	}

	pub fn dev_item (& self) -> BtrfsDevItem <'a> {
		BtrfsDevItem::new (& self.data.dev_item)
	}

	pub fn label (& self) -> & BtrfsLabel {
		& self.data.label
	}

	pub fn cache_generation (& self) -> u64 {
		self.data.cache_generation
	}

	pub fn uuid_tree_generation (& self) -> u64 {
		self.data.uuid_tree_generation
	}

	pub fn root_backups (& self) -> & [BtrfsRootBackup] {
		& self.data.root_backups
	}

	pub fn device_id (& self) -> u64 {
		self.dev_item ().device_id ()
	}

}

impl <'a> Debug for BtrfsSuperblock <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsSuperblock");

		debug_struct.field (
			"checksum",
			& self.checksum ());

		debug_struct.field (
			"fs_uuid",
			& self.fs_uuid ());

		debug_struct.field (
			"physical_address",
			& self.physical_address ());

		debug_struct.field (
			"flags",
			& self.flags ());

		debug_struct.field (
			"magic",
			& self.magic ());

		debug_struct.field (
			"generation",
			& self.generation ());

		debug_struct.field (
			"root_tree_logical_address",
			& self.root_tree_logical_address ());

		debug_struct.field (
			"chunk_tree_logical_address",
			& self.chunk_tree_logical_address ());

		debug_struct.field (
			"log_tree_logical_address",
			& self.log_tree_logical_address ());

		debug_struct.field (
			"log_root_transid",
			& self.log_root_transid ());

		debug_struct.field (
			"total_bytes",
			& self.total_bytes ());

		debug_struct.field (
			"bytes_used",
			& self.bytes_used ());

		debug_struct.field (
			"root_dir_objectid",
			& self.root_dir_objectid ());

		debug_struct.field (
			"num_devices",
			& self.num_devices ());

		debug_struct.field (
			"sector_size",
			& self.sector_size ());

		debug_struct.field (
			"node_size",
			& self.node_size ());

		debug_struct.field (
			"leaf_size",
			& self.leaf_size ());

		debug_struct.field (
			"stipe_size",
			& self.stripe_size ());

		debug_struct.field (
			"system_chunks_size",
			& self.system_chunks_size ());

		debug_struct.field (
			"chunk_root_generation",
			& self.chunk_root_generation ());

		debug_struct.field (
			"compat_flags",
			& self.compat_flags ());

		debug_struct.field (
			"compat_ro_flags",
			& self.compat_ro_flags ());

		debug_struct.field (
			"incompat_flags",
			& self.incompat_flags ());

		debug_struct.field (
			"csum_type",
			& self.csum_type ());

		debug_struct.field (
			"root_level",
			& self.root_level ());

		debug_struct.field (
			"chunk_root_level",
			& self.chunk_root_level ());

		debug_struct.field (
			"log_root_level",
			& self.log_root_level ());

		debug_struct.field (
			"dev_item",
			& self.dev_item ());

		debug_struct.field (
			"label",
			& self.label ());

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

// ex: noet ts=4 filetype=rust
