use std::mem;

use diskformat::*;

pub struct BtrfsLeafNodeItems <'a> {
	bytes: & 'a [u8],
	index: u32,
	num_items: u32,
}

impl <'a> BtrfsLeafNodeItems <'a> {

	pub fn new (
		bytes: & 'a [u8],
		num_items: u32,
	) -> BtrfsLeafNodeItems <'a> {

		BtrfsLeafNodeItems {
			bytes: bytes,
			index: 0,
			num_items: num_items,
		}

	}

}

impl <'a> Iterator for BtrfsLeafNodeItems <'a> {

	type Item = BtrfsLeafItem <'a>;

	fn next (
		& mut self,
	) -> Option <BtrfsLeafItem <'a>> {

		if self.index < self.num_items {

			// read header

			let header_start =
				self.index as usize
					* mem::size_of::<BtrfsLeafItemHeader> ();

			let header_end =
				self.index as usize
					* mem::size_of::<BtrfsLeafItemHeader> ()
				+ mem::size_of::<BtrfsLeafItemHeader> ();

			let item_header_bytes =
				& self.bytes [
					header_start .. header_end];

			let item_header =
				BtrfsLeafItemHeader::from_bytes (
					item_header_bytes,
				).unwrap ();

			// read data

			let item_data =
				& self.bytes [
					item_header.data_offset () as usize
				..
					item_header.data_offset () as usize
					+ item_header.data_size () as usize
				];

			self.index += 1;

			Some (
				BtrfsLeafItem::from_bytes (
					item_header,
					item_data)
			)

		} else {

			None

		}

	}

}

// ex: noet ts=4 filetype=rust
