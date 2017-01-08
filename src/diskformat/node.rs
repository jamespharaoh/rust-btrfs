use std::mem;

use diskformat::*;

#[ derive (Clone) ]
pub enum BtrfsNode <'a> {
	Internal (BtrfsInternalNode <'a>),
	Leaf (BtrfsLeafNode <'a>),
}

#[ derive (Clone) ]
pub struct BtrfsInternalNode <'a> {
	position: usize,
	bytes: & 'a [u8],
}

#[ derive (Clone) ]
pub struct BtrfsLeafNode <'a> {
	position: usize,
	bytes: & 'a [u8],
}

pub struct BtrfsLeafNodeItems <'a> {
	node: BtrfsLeafNode <'a>,
	index: u32,
}

impl <'a> BtrfsNode <'a> {

	pub fn from_bytes (
		position: usize,
		bytes: & 'a [u8],
	) -> Result <BtrfsNode <'a>, String> {

		// verify checksum

		let calculated_checksum =
			BtrfsChecksum::for_bytes (
				& bytes [0x20 .. bytes.len ()]);

		let header = unsafe {
			& * (bytes.as_ptr () as * const BtrfsNodeHeader)
		};

		if header.checksum () != calculated_checksum {

			return Err (
				"Checksum mismatch".to_owned ());

		}

		// construct

		if header.level () == 0 {

			Ok (BtrfsNode::Leaf (
				BtrfsLeafNode {
					position: position,
					bytes: bytes,
				}
			))

		} else {

			Ok (BtrfsNode::Internal (
				BtrfsInternalNode {
					position: position,
					bytes: bytes,
				}
			))

		}

	}

}

impl <'a> BtrfsLeafNode <'a> {

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
		self.header ().checksum
	}

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.header ().fs_uuid
	}

	pub fn tree_id (& self) -> u64 {
		self.header ().tree_id
	}

	pub fn num_items (& self) -> u32 {
		self.header ().num_items
	}

	pub fn level (& self) -> u8 {
		self.header ().level
	}

}

impl <'a> Iterator for BtrfsLeafNodeItems <'a> {

	type Item = BtrfsItem <'a>;

	fn next (
		& mut self,
	) -> Option <BtrfsItem <'a>> {

		if self.index < self.node.num_items () {

			// read header

			let header_start =
				mem::size_of::<BtrfsNodeHeader> ()
				+ self.index as usize
					* mem::size_of::<BtrfsItemHeader> ();

			let header_end =
				mem::size_of::<BtrfsNodeHeader> ()
				+ self.index as usize
					* mem::size_of::<BtrfsItemHeader> ()
				+ mem::size_of::<BtrfsItemHeader> ();

			let item_header_bytes =
				& self.node.bytes [
					header_start .. header_end];

			let item_header =
				BtrfsItemHeader::from_bytes (
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
				BtrfsItem::from_bytes (
					item_header,
					item_data)
			)

		} else {

			None

		}

	}

}

impl <'a> BtrfsInternalNode <'a> {

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

}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, PartialEq) ]
pub struct BtrfsNodeHeader {
	checksum: BtrfsChecksum,
	fs_uuid: BtrfsUuid,
	logical_address: u64,
	flags_and_backref: u64,
	chunk_tree_uuid: BtrfsUuid,
	generation: u64,
	tree_id: u64,
	num_items: u32,
	level: u8,
}

impl BtrfsNodeHeader {

	pub fn checksum (& self) -> BtrfsChecksum {
		self.checksum
	}

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.fs_uuid
	}

	pub fn tree_id (& self) -> u64 {
		self.tree_id
	}

	pub fn num_items (& self) -> u32 {
		self.num_items
	}

	pub fn level (& self) -> u8 {
		self.level
	}

}

// ex: noet ts=4 filetype=rust
