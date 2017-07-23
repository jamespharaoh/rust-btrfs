use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use super::super::*;

#[ derive (Clone, Eq, Hash, PartialEq) ]
pub enum BtrfsNode <'a> {
	Internal (BtrfsInternalNode <'a>),
	Leaf (BtrfsLeafNode <'a>),
}

impl <'a> BtrfsNode <'a> {

	pub fn from_bytes (
		physical_address: BtrfsPhysicalAddress,
		bytes: & 'a [u8],
	) -> Result <BtrfsNode <'a>, String> {

		// verify checksum

		let calculated_checksum =
			BtrfsChecksum::from (
				btrfs_crc32c (
					& bytes [0x20 .. bytes.len ()]));

		let header = unsafe {
			& * (bytes.as_ptr () as * const BtrfsNodeHeader)
		};

		if header.checksum () != calculated_checksum {

			return Err (
				"Checksum mismatch".to_owned ());

		}

		// construct

		if header.level () == 0 {

			Ok (
				BtrfsNode::Leaf (
					BtrfsLeafNode::new (
						physical_address,
						bytes,
					)
				)
			)

		} else {

			Ok (
				BtrfsNode::Internal (
					BtrfsInternalNode::new (
						physical_address,
						bytes,
					) ?
				)
			)

		}

	}

	pub fn physical_address (
		& self,
	) -> BtrfsPhysicalAddress {

		match self {

			& BtrfsNode::Internal (ref node) =>
				node.physical_address (),

			& BtrfsNode::Leaf (ref node) =>
				node.physical_address (),

		}

	}

	pub fn header (
		& self,
	) -> & BtrfsNodeHeader {

		match self {

			& BtrfsNode::Internal (ref node) =>
				node.header (),

			& BtrfsNode::Leaf (ref node) =>
				node.header (),

		}

	}

	pub fn tree_id (& self) -> BtrfsTreeId {
		self.header ().tree_id ()
	}

	pub fn generation (& self) -> u64 {
		self.header ().generation ()
	}

}

impl <'a> Debug for BtrfsNode <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		match self {

			& BtrfsNode::Internal (internal_node) =>
				formatter.write_fmt (
					format_args! (
						"{:?}",
						internal_node)),

			& BtrfsNode::Leaf (leaf_node) =>
				formatter.write_fmt (
					format_args! (
						"{:?}",
						leaf_node)),

		}

	}

}

// ex: noet ts=4 filetype=rust
