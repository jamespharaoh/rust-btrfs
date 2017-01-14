use std::mem;

use diskformat::*;

#[ derive (Clone) ]
pub struct BtrfsInternalNode <'a> {
	/*position: usize,*/
	bytes: & 'a [u8],
}

pub struct BtrfsInternalNodeItems <'a> {
	node: BtrfsInternalNode <'a>,
	index: u32,
}

impl <'a> BtrfsInternalNode <'a> {

	pub fn new (
		/*position: usize,*/
		bytes: & 'a [u8],
	) -> BtrfsInternalNode <'a> {

		BtrfsInternalNode {
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
		false
	}

	pub fn items (
		self,
	) -> BtrfsInternalNodeItems <'a> {

		BtrfsInternalNodeItems {
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

impl <'a> Iterator for BtrfsInternalNodeItems <'a> {

	type Item = & 'a BtrfsInternalItem;

	fn next (
		& mut self,
	) -> Option <& 'a BtrfsInternalItem> {

		if self.index < self.node.num_items () {

			let item_start =
				mem::size_of::<BtrfsNodeHeader> ()
				+ self.index as usize
					* mem::size_of::<BtrfsInternalItem> ();

			let item = unsafe {
				& * (
					item_start
						as * const u8
						as * const BtrfsInternalItem
				)
			};

			self.index += 1;

			Some (item)

		} else {

			None

		}

	}

}

// ex: noet ts=4 filetype=rust
