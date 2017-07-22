use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;
use std::slice;

use super::super::*;

#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
pub struct BtrfsInternalNode <'a> {
	physical_address: BtrfsPhysicalAddress,
	bytes: & 'a [u8],
}

impl <'a> BtrfsInternalNode <'a> {

	pub fn new (
		physical_address: BtrfsPhysicalAddress,
		bytes: & 'a [u8],
	) -> Result <BtrfsInternalNode <'a>, String> {

		Ok (BtrfsInternalNode {
			physical_address: physical_address,
			bytes: bytes,
		})

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
		& self,
	) -> & 'a [BtrfsInternalItem] {

		let start_address =
			& self.bytes [mem::size_of::<BtrfsNodeHeader> ()]
				as * const u8
				as * const BtrfsInternalItem;

		unsafe {

			slice::from_raw_parts (
				start_address,
				self.header ().num_items () as usize)

		}

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

impl <'a> Debug for BtrfsInternalNode <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsInternalNode");

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
