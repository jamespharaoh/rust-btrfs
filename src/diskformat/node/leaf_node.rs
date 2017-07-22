use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use diskformat::*;

#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
pub struct BtrfsLeafNode <'a> {
	physical_address: BtrfsPhysicalAddress,
	bytes: & 'a [u8],
}

impl <'a> BtrfsLeafNode <'a> {

	pub fn new (
		physical_address: BtrfsPhysicalAddress,
		bytes: & 'a [u8],
	) -> BtrfsLeafNode <'a> {

		BtrfsLeafNode {
			physical_address: physical_address,
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

		BtrfsLeafNodeItems::new (
			& self.bytes [mem::size_of::<BtrfsNodeHeader> () ..],
			self.num_items (),
		)

	}

	pub fn physical_address (& self) -> BtrfsPhysicalAddress {
		self.physical_address
	}

	pub fn checksum (& self) -> BtrfsChecksum {
		self.header ().checksum ()
	}

	pub fn fs_uuid (& self) -> BtrfsUuid {
		self.header ().fs_uuid ()
	}

	pub fn tree_id (& self) -> BtrfsTreeId {
		self.header ().tree_id ()
	}

	pub fn num_items (& self) -> u32 {
		self.header ().num_items ()
	}

	pub fn level (& self) -> u8 {
		self.header ().level ()
	}

}

impl <'a> Debug for BtrfsLeafNode <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsLeafNode");

		self.header ().debug_struct (
			& mut debug_struct);

		debug_struct.field (
			"items",
			& NakedString::from (
				"..."));

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
