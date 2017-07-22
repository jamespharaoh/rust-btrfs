use std::fmt::Debug;
use std::fmt::DebugStruct;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use diskformat::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsLeafItemHeader {
	key: BtrfsKey,
	data_offset: u32,
	data_size: u32,
}

impl BtrfsLeafItemHeader {

	pub fn from_bytes (
		bytes: & [u8],
	) -> Result <& BtrfsLeafItemHeader, String> {

		// sanity check

		if bytes.len () != mem::size_of::<BtrfsLeafItemHeader> () {

			return Err (
				format! (
					"Must be exactly 0x{:x} bytes",
					mem::size_of::<BtrfsLeafItemHeader> ()));

		}

		// cast

		Ok (
			unsafe {
				& * (bytes.as_ptr () as * const BtrfsLeafItemHeader)
			}
		)

	}

	pub fn key (& self) -> BtrfsKey {
		self.key
	}

	pub fn object_id (& self) -> u64 {
		self.key.object_id ()
	}

	pub fn item_type (& self) -> u8 {
		self.key.item_type ()
	}

	pub fn offset (& self) -> u64 {
		self.key.offset ()
	}

	pub fn data_offset (& self) -> u32 {
		self.data_offset
	}

	pub fn data_size (& self) -> u32 {
		self.data_size
	}

	pub fn debug_struct (
		& self,
		debug_struct: & mut DebugStruct,
	) {

		debug_struct.field (
			"key",
			& NakedString::from (
				self.key ().to_string ()));

		debug_struct.field (
			"data_offset",
			& NakedString::from (
				format! (
					"0x{:x}",
					self.data_offset ())));

		debug_struct.field (
			"data_size",
			& self.data_size ());

	}

}

impl Debug for BtrfsLeafItemHeader {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsLeafItemHeader");

		self.debug_struct (
			& mut debug_struct);

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
