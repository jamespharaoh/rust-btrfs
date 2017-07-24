use super::super::prelude::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsExtentData <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

impl <'a> BtrfsLeafItemContents <'a> for BtrfsExtentData <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

impl <'a> BtrfsExtentData <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsExtentData <'a>, String> {

		// create extent data

		let extent_data =
			BtrfsExtentData {
				header: header,
				data_bytes: data_bytes,
			};

		// sanity check

		if extent_data.inline () {

			if data_bytes.len ()
				< mem::size_of::<BtrfsExtentDataData> ()
					- mem::size_of::<u64> () * 4 {

				return Err (
					format! (
						"Must be at least 0x{:x} bytes",
						mem::size_of::<BtrfsExtentDataData> ()
							- mem::size_of::<u64> () * 4));

			}

		} else {

			if data_bytes.len ()
				!= mem::size_of::<BtrfsExtentDataData> () {

				return Err (
					format! (
						"Must be exactly 0x{:x} bytes",
						mem::size_of::<BtrfsExtentDataData> ()));

			}

		}

		// return

		Ok (extent_data)

	}

	pub fn offset (& self) -> u64 {
		self.header.offset ()
	}

	pub fn data (& self) -> & BtrfsExtentDataData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsExtentDataData
			)
		}

	}

	pub fn generation (& self) -> u64 {
		self.data ().generation
	}

	pub fn logical_data_size (& self) -> u64 {
		self.data ().logical_data_size
	}

	pub fn compression (& self) -> u8 {
		self.data ().compression
	}

	pub fn encryption (& self) -> u8 {
		self.data ().encryption
	}

	pub fn other_encoding (& self) -> u16 {
		self.data ().other_encoding
	}

	pub fn extent_type (& self) -> u8 {
		self.data ().extent_type
	}

	pub fn extent_logical_address (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().extent_logical_address

	}

	pub fn extent_size (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().extent_size

	}

	pub fn extent_data_offset (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().extent_data_offset

	}

	pub fn extent_data_size (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().extent_data_size

	}

	pub fn inline (& self) -> bool {
		self.data ().extent_type == BTRFS_EXTENT_DATA_INLINE_TYPE
	}

	pub fn regular (& self) -> bool {
		self.data ().extent_type == BTRFS_EXTENT_DATA_REGULAR_TYPE
	}

	pub fn prealloc (& self) -> bool {
		self.data ().extent_type == BTRFS_EXTENT_DATA_PREALLOC_TYPE
	}

	pub fn inline_data (
		& self,
	) -> Result <Option <Cow <[u8]>>, String> {

		if self.inline () {

			let raw_data_start =
				mem::size_of::<BtrfsExtentDataData> ()
				- 4 * mem::size_of::<u64> ();

			let raw_data_end =
				self.data_bytes.len ();

			let raw_data =
				& self.data_bytes [raw_data_start .. raw_data_end];

			if self.data ().encryption != 0 {

				return Err (
					format! (
						"Unrecognised encryption type {}",
						self.data ().encryption)
				);

			}

			if self.data ().other_encoding != 0 {

				return Err (
					format! (
						"Unrecognised other encoding type {}",
						self.data ().other_encoding)
				);

			}

			btrfs_decompress_pages (
				self.compression (),
				raw_data,
				self.logical_data_size (),
			).map (Some)

		} else {

			Ok (None)

		}

	}

}

// ex: noet ts=4 filetype=rust
