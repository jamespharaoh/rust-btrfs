use std::mem;

use super::super::*;

#[ derive (Clone) ]
pub struct BtrfsDevice <'a> {
	data: & 'a [u8],
	superblock_data: * const BtrfsSuperblockData,
	superblock_offset: usize,
}

impl <'a> BtrfsDevice <'a> {

	pub fn new (
		data: & [u8],
	) -> Result <BtrfsDevice, String> {

		// find superblock

		let (superblock_data, superblock_offset) = {

			let mut superblock_datas: Vec <(& BtrfsSuperblockData, usize)> =
				Vec::new ();

			for superblock_offset in vec! [
				0x1_0000,
				0x400_0000,
				0x40_0000_0000,
				0x4_0000_0000_0000,
			] {

				if superblock_offset
					+ mem::size_of::<BtrfsSuperblockData> ()
						>= data.len () {

					continue;

				}

				let superblock_data: & BtrfsSuperblockData =
					unsafe {
						mem::transmute (
							& data [superblock_offset])
					};

				if superblock_data.magic != BTRFS_MAGIC {
					continue;
				}

				superblock_datas.push (
					(
						superblock_data,
						superblock_offset,
					)
				);

			}

			superblock_datas.into_iter ().max_by_key (
				|& (superblock_data, _offset)| superblock_data.generation,
			).ok_or (
				"No superblocks found".to_string (),
			) ?

		};

		Ok (BtrfsDevice {
			data: data,
			superblock_data: superblock_data,
			superblock_offset: superblock_offset,
		})

	}

	pub fn slice_at (
		& self,
		offset: usize,
		len: usize,
	) -> Option <& [u8]> {

		if offset + len <= self.data.len () {

			Some (
				& self.data [offset .. offset + len]
			)

		} else {

			None

		}

	}

	pub unsafe fn struct_at <Type> (
		& self,
		offset: u64,
	) -> & Type {

		mem::transmute (
			& self.data [
				offset as usize])

	}

	pub fn node_at (
		& self,
		offset: u64,
	) -> Result <BtrfsNode, String> {

		self.slice_at (
			offset as usize,
			self.superblock ().node_size () as usize,
		).ok_or (

			format! (
				"Node offset out of range for device {}: 0x{:x}",
				self.device_id (),
				offset)

		).and_then (
			|node_bytes|

			BtrfsNode::from_bytes (
				BtrfsPhysicalAddress::new (
					self.device_id (),
					offset as u64),
				node_bytes)

		)

	}

	pub fn data (& self) -> & [u8] {
		self.data
	}

	pub fn len (& self) -> usize {
		self.data.len ()
	}

	pub fn superblock (
		& self,
	) -> BtrfsSuperblock {

		BtrfsSuperblock::new (
			unsafe {
				& * self.superblock_data
			}
		)

	}

	pub fn superblock_offset (& self) -> usize {
		self.superblock_offset
	}

	pub fn device_id (& self) -> u64 {
		self.superblock ().device_id ()
	}

}

// ex: noett ts=4 filetype=rust
