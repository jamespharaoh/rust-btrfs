use std::mem;

use diskformat::*;

#[ repr (C, packed) ]
#[ derive (Copy) ]
pub struct BtrfsSuperblock {
	checksum: BtrfsChecksum,
	fs_uuid: BtrfsUuid,
	physical_address: u64,
	flags: u64,
	magic: [u8; 0x8],
	generation: u64,
	root_tree_logical_address: u64,
	chunk_tree_logical_address: u64,
	log_tree_logical_address: u64,
	log_root_transid: u64,
	total_bytes: u64,
	bytes_used: u64,
	root_dir_objectid: u64,
	num_devices: u64,
	sector_size: u32,
	node_size: u32,
	leaf_size: u32,
	stripesize: u32,
	system_chunks_size: u32,
	chunk_root_generation: u64,
	compat_flags: u64,
	compat_ro_flags: u64,
	incompat_flags: u64,
	csum_type: u16,
	root_level: u8,
	chunk_root_level: u8,
	log_root_level: u8,
	dev_item: BtrfsDevItem,
	label: BtrfsLabel,
	reserved: [u8; 0x100],
	system_chunks: [u8; 0x800],
	unused: [u8; 0x4d5],
}

pub struct BtrfsSuperblockSystemChunks <'a> {
	_superblock: & 'a BtrfsSuperblock,
	address: * const u8,
	end_address: * const u8,
}

impl BtrfsSuperblock {

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.fs_uuid
	}

	pub fn system_chunks (& self) -> BtrfsSuperblockSystemChunks {

		let start_address =
			self.system_chunks.as_ptr ();

		let end_address = unsafe {
			start_address.offset (
				self.system_chunks_size as isize,
			)
		};

		BtrfsSuperblockSystemChunks {
			_superblock: self,
			address: start_address,
			end_address: end_address,
		}

	}

	pub fn system_logical_to_physical (
		& self,
		logical_address: u64,
	) -> Option <(u64, u64)> {

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
				(
					system_chunk_stripe.device_id (),
					logical_address
						- system_chunk.key ().offset ()
						+ system_chunk_stripe.offset (),
				)
			);

		}

		None

	}

	pub fn magic (& self) -> [u8; 0x8] {
		self.magic
	}

	pub fn root_tree_logical_address (& self) -> u64 {
		self.root_tree_logical_address
	}

	pub fn chunk_tree_logical_address (& self) -> u64 {
		self.chunk_tree_logical_address
	}

	pub fn sector_size (& self) -> u32 {
		self.sector_size
	}

	pub fn node_size (& self) -> u32 {
		self.node_size
	}

	pub fn leaf_size (& self) -> u32 {
		self.leaf_size
	}

	pub fn dev_item (& self) -> & BtrfsDevItem {
		& self.dev_item
	}

}

impl Clone for BtrfsSuperblock {

	fn clone (
		& self,
	) -> Self {
		* self
	}

}

impl <'a> Iterator for BtrfsSuperblockSystemChunks <'a> {

	type Item = BtrfsChunkItemSimple <'a>;

	fn next (
		& mut self,
	) -> Option <BtrfsChunkItemSimple <'a>> {

		if self.address < self.end_address {

			// get key

			let chunk_item_key = unsafe {
				& * (
					self.address
						as * const BtrfsKey
				)
			};

			self.address = unsafe {
				self.address.offset (
					mem::size_of::<BtrfsKey> () as isize,
				)
			};

			// get data

			let chunk_item_data = unsafe {
				& * (
					self.address
						as * const BtrfsChunkItemData
				)
			};

			self.address = unsafe {
				self.address.offset (
					mem::size_of::<BtrfsChunkItemData> () as isize,
				)
			};

			// skip chunk item stripes

			self.address = unsafe {
				self.address.offset (
					mem::size_of::<BtrfsChunkItemStripeData> () as isize
						* chunk_item_data.num_stripes () as isize,
				)
			};

			// return

			Some (
				BtrfsChunkItemSimple::new (
					chunk_item_key,
					chunk_item_data,
				).expect ("Invalid chunk item")
			)

		} else {

			None

		}

	}

}

// ex: noet ts=4 filetype=rust
