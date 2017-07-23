use std::mem;
use std::slice;

use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug) ]
pub struct BtrfsChunkItemData {
	pub chunk_size: u64,
	pub root_object_id: u64,
	pub stripe_length: u64,
	pub flags: u64,
	pub optimal_io_alignment: u32,
	pub optimal_io_width: u32,
	pub minimal_io_size: u32,
	pub num_stripes: u16,
	pub sub_stripes: u16,
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

// ex: noet ts=4 filetype=rust
