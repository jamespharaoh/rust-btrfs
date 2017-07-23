use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use diskformat::*;

#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsDirItemEntry <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

impl <'a> BtrfsDirItemEntry <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsDirItemEntry <'a>, String> {

		// sanity check

		if data_bytes.len () < mem::size_of::<BtrfsDirItemData> () {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsDirItemData> ()));

		}

		// create dir item

		let dir_item = BtrfsDirItemEntry {
			header: header,
			data_bytes: data_bytes,
		};

		// sanity check

		if data_bytes.len () != (
			mem::size_of::<BtrfsDirItemData> ()
			+ dir_item.data_size () as usize
			+ dir_item.name_size () as usize
		) {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsDirItemData> ()
					+ dir_item.data_size () as usize
					+ dir_item.name_size () as usize));

		}

		// return

		Ok (dir_item)

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

	pub fn data (& self) -> & BtrfsDirItemData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsDirItemData
			)
		}

	}

	pub fn child_key (& self) -> BtrfsKey {
		self.data ().child_key
	}

	pub fn child_object_id (& self) -> u64 {
		self.data ().child_key.object_id ()
	}

	pub fn transaction_id (& self) -> u64 {
		self.data ().transaction_id
	}

	pub fn name_size (& self) -> u16 {
		self.data ().name_size
	}

	pub fn data_size (& self) -> u16 {
		self.data ().data_size
	}

	pub fn child_type (& self) -> u8 {
		self.data ().child_type
	}

	pub fn name (
		& 'a self,
	) -> & 'a [u8] {

		& self.data_bytes [
			mem::size_of::<BtrfsDirItemData> ()
		..
			mem::size_of::<BtrfsDirItemData> ()
			+ self.name_size () as usize
		]

	}

	pub fn name_to_string_lossy (
		& self,
	) -> Cow <str> {

		String::from_utf8_lossy (
			self.name ())

	}

}

impl <'a> BtrfsLeafItemContents <'a> for BtrfsDirItemEntry <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

impl <'a> Debug for BtrfsDirItemEntry <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsDirItemEntry");

		self.header ().debug_struct (
			& mut debug_struct);

		debug_struct.field (
			"child_key",
			& NakedString::from (
				self.child_key ().to_string ()));

		debug_struct.field (
			"transaction_id",
			& self.transaction_id ());

		debug_struct.field (
			"data_size",
			& self.data_size ());

		debug_struct.field (
			"name",
			& self.name_to_string_lossy ());

		debug_struct.field (
			"child_type",
			& self.child_type ());

		debug_struct.field (
			"data",
			& NakedString::from (
				"..."));

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
