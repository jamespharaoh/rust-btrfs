use std::mem;

use diskformat::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsDirIndex <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsDirIndexData {
	child_key: BtrfsKey,
	transaction_id: u64,
	data_size: u16,
	name_size: u16,
	child_type: u8,
}

impl <'a> BtrfsDirIndex <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsDirIndex <'a>, String> {

		// sanity check

		if data_bytes.len () < mem::size_of::<BtrfsDirIndexData> () {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsDirIndexData> ()));

		}

		// create dir item

		let dir_item = BtrfsDirIndex {
			header: header,
			data_bytes: data_bytes,
		};

		// sanity check

		if data_bytes.len () != (
			mem::size_of::<BtrfsDirIndexData> ()
			+ dir_item.data_size () as usize
			+ dir_item.name_size () as usize
		) {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsDirIndexData> ()
					+ dir_item.data_size () as usize
					+ dir_item.name_size () as usize));

		}

		// return

		Ok (dir_item)

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

	pub fn object_id (& self) -> u64 {
		self.header.object_id ()
	}

	pub fn key (& self) -> BtrfsKey {
		self.header.key ()
	}

	pub fn data (& self) -> & BtrfsDirIndexData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsDirIndexData
			)
		}

	}

	pub fn child_key (& self) -> BtrfsKey {
		self.data ().child_key
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
			mem::size_of::<BtrfsDirIndexData> ()
		..
			mem::size_of::<BtrfsDirIndexData> ()
			+ self.name_size () as usize
		]

	}

}

// ex: noet ts=4 filetype=rust
