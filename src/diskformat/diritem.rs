use std::mem;

use diskformat::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsDirItem <'a> {
	header: & 'a BtrfsItemHeader,
	data_bytes: & 'a [u8],
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsDirItemData {
	child_key: BtrfsKey,
	transaction_id: u64,
	data_size: u16,
	name_size: u16,
	child_type: u8,
}

impl <'a> BtrfsDirItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsDirItem <'a>, String> {

		// sanity check

		if data_bytes.len () < mem::size_of::<BtrfsDirItemData> () {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsDirItemData> ()));

		}

		// create dir item

		let dir_item = BtrfsDirItem {
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

	pub fn header (& self) -> & BtrfsItemHeader {
		self.header
	}

	pub fn key (& self) -> BtrfsKey {
		self.header.key ()
	}

	pub fn object_id (& self) -> u64 {
		self.header.object_id ()
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

}

// ex: noet ts=4 filetype=rust
