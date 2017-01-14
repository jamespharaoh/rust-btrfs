use std::mem;

use diskformat::*;

#[ derive (Clone) ]
pub struct BtrfsLeafNode <'a> {
	/*position: usize,*/
	bytes: & 'a [u8],
}

pub struct BtrfsLeafNodeItems <'a> {
	node: BtrfsLeafNode <'a>,
	index: u32,
}

impl <'a> BtrfsLeafNode <'a> {

	pub fn new (
		/*position: usize,*/
		bytes: & 'a [u8],
	) -> BtrfsLeafNode <'a> {

		BtrfsLeafNode {
			/*position: position,*/
			bytes: bytes,
		}

	}

	pub fn header (
		& 'a self,
	) -> & 'a BtrfsNodeHeader {

		unsafe {
			& * (self.bytes.as_ptr () as * const BtrfsNodeHeader)
		}

	}

	pub fn is_leaf (
		& self,
	) -> bool {
		self.header ().level () == 0
	}

	pub fn items (
		self,
	) -> BtrfsLeafNodeItems <'a> {

		BtrfsLeafNodeItems {
			node: self,
			index: 0,
		}

	}

	pub fn checksum (& self) -> BtrfsChecksum {
		self.header ().checksum ()
	}

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.header ().fs_uuid ()
	}

	pub fn tree_id (& self) -> u64 {
		self.header ().tree_id ()
	}

	pub fn num_items (& self) -> u32 {
		self.header ().num_items ()
	}

	pub fn level (& self) -> u8 {
		self.header ().level ()
	}

}

impl <'a> Iterator for BtrfsLeafNodeItems <'a> {

	type Item = BtrfsLeafItem <'a>;

	fn next (
		& mut self,
	) -> Option <BtrfsLeafItem <'a>> {

		if self.index < self.node.num_items () {

			// read header

			let header_start =
				mem::size_of::<BtrfsNodeHeader> ()
				+ self.index as usize
					* mem::size_of::<BtrfsLeafItemHeader> ();

			let header_end =
				mem::size_of::<BtrfsNodeHeader> ()
				+ self.index as usize
					* mem::size_of::<BtrfsLeafItemHeader> ()
				+ mem::size_of::<BtrfsLeafItemHeader> ();

			let item_header_bytes =
				& self.node.bytes [
					header_start .. header_end];

			let item_header =
				BtrfsLeafItemHeader::from_bytes (
					item_header_bytes,
				).unwrap ();

			// read data

			let item_data =
				& self.node.bytes [
					mem::size_of::<BtrfsNodeHeader> ()
					+ item_header.data_offset () as usize
				..
					mem::size_of::<BtrfsNodeHeader> ()
					+ item_header.data_offset () as usize
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
