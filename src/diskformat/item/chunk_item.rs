use std::mem;
use std::slice;

use diskformat::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsChunkItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug) ]
pub struct BtrfsChunkItemData {
	chunk_size: u64,
	root_object_id: u64,
	stripe_length: u64,
	flags: u64,
	optimal_io_alignment: u32,
	optimal_io_width: u32,
	minimal_io_size: u32,
	num_stripes: u16,
	sub_stripes: u16,
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug) ]
pub struct BtrfsChunkItemStripeData {
	device_id: u64,
	offset: u64,
	device_uuid: BtrfsUuid,
}

impl <'a> BtrfsChunkItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsChunkItem <'a>, String> {

		// sanity check

		if data_bytes.len () < mem::size_of::<BtrfsChunkItemData> () {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsInodeItemData> ()));

		}

		// TODO check stripes

		// create chunk item

		Ok (
			BtrfsChunkItem {
				header: header,
				data_bytes: data_bytes,
			}
		)

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

	pub fn object_id (& self) -> u64 {
		self.header.object_id ()
	}

	pub fn key (& self) -> BtrfsKey {
		self.header.key ()
	}

	pub fn data (& self) -> & BtrfsChunkItemData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsChunkItemData
			)
		}

	}

	pub fn stripes (& self) -> & [BtrfsChunkItemStripeData] {
		self.data ().stripes ()
	}

}

impl BtrfsChunkItemData {

	pub fn chunk_size (& self) -> u64 {
		self.chunk_size
	}

	pub fn stripes (& self) -> & [BtrfsChunkItemStripeData] {

		unsafe {
			slice::from_raw_parts (
				(
					self
						as * const BtrfsChunkItemData
						as * const u8
				).offset (
					mem::size_of::<BtrfsChunkItemData> () as isize,
				) as * const BtrfsChunkItemStripeData,
				self.num_stripes as usize,
			)
		}

	}

	pub fn num_stripes (& self) -> u16 {
		self.num_stripes
	}

	pub fn sub_stripes (& self) -> u16 {
		self.sub_stripes
	}

}

impl BtrfsChunkItemStripeData {

	pub fn device_id (& self) -> u64 {
		self.device_id
	}

	pub fn offset (& self) -> u64 {
		self.offset
	}

	pub fn device_uuid (& self) -> BtrfsUuid {
		self.device_uuid
	}

}


// ex: noet ts=4 filetype=rust
