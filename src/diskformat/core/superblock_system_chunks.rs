use std::marker::PhantomData;
use std::mem;

use super::super::*;

pub struct BtrfsSuperblockSystemChunks <'a> {
	address: * const u8,
	end_address: * const u8,
	phantom: PhantomData <& 'a BtrfsSuperblockData>,
}

impl <'a> BtrfsSuperblockSystemChunks <'a> {

	pub fn new (
		superblock_data: & 'a BtrfsSuperblockData,
	) -> BtrfsSuperblockSystemChunks <'a> {

		let start_address =
			& superblock_data.system_chunks.data [0] as * const u8;

		let end_address =
			unsafe {

			start_address.offset (
				superblock_data.system_chunks_size as isize)

		};

		BtrfsSuperblockSystemChunks {
			address: start_address,
			end_address: end_address,
			phantom: PhantomData,
		}

	}

}

impl <'a> Iterator for BtrfsSuperblockSystemChunks <'a> {

	type Item = BtrfsSuperblockChunkItem <'a>;

	fn next (
		& mut self,
	) -> Option <BtrfsSuperblockChunkItem <'a>> {

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
				BtrfsSuperblockChunkItem::new (
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
