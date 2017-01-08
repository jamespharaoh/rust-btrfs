use std::borrow::Cow;
use std::io;
use std::io::Write;
use std::mem;

use flate2;

use minilzo;

use diskformat::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsExtentData <'a> {
	header: & 'a BtrfsItemHeader,
	data_bytes: & 'a [u8],
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsExtentDataData {

	generation: u64,
	logical_data_size: u64,
	compression: u8,
	encryption: u8,
	other_encoding: u16,
	extent_type: u8,

	logical_address: u64,
	extent_size: u64,
	extent_offset: u64,
	logical_bytes: u64,

}

impl <'a> BtrfsExtentData <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsItemHeader,
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

	pub fn header (& self) -> & BtrfsItemHeader {
		self.header
	}

	pub fn key (& self) -> BtrfsKey {
		self.header.key ()
	}

	pub fn object_id (& self) -> u64 {
		self.header.object_id ()
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

	pub fn logical_address (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().logical_address

	}

	pub fn extent_size (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().extent_size

	}

	pub fn extent_offset (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().extent_offset

	}

	pub fn logical_bytes (& self) -> u64 {

		if self.inline () {
			panic! ();
		}

		self.data ().logical_bytes

	}

	pub fn inline (
		& self,
	) -> bool {
		self.data ().extent_type == BTRFS_EXTENT_DATA_INLINE_TYPE
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

			match self.data ().compression {

				BTRFS_EXTENT_DATA_NO_COMPRESSION =>
					Ok (Some (
						Cow::Borrowed (
							raw_data)
					)),

				BTRFS_EXTENT_DATA_LZO_COMPRESSION => try! (
					minilzo::decompress (
						raw_data,
						self.data ().logical_data_size as usize,
					).map (
						|uncompressed_data|

{
io::stdout ().write_fmt (
	format_args! (
		"LZO SUCCESS {} -> {} bytes\r\n",
		raw_data.len (),
		uncompressed_data.len ()));
						Ok (Some (
							Cow::Owned (
								uncompressed_data)
						))
}

					).or_else (
						|error|

						Err (
							format! (
								"LZO decompression failed: {:?}",
								error)
						)

					)
				),

				BTRFS_EXTENT_DATA_ZLIB_COMPRESSION => {

					let mut uncompressed_data =
						Vec::with_capacity (
							self.data ().logical_data_size as usize);

					uncompressed_data.resize (
						self.data ().logical_data_size as usize,
						0u8);

					let mut decompress =
						flate2::Decompress::new (
							false);

					match (
						decompress.decompress (
							raw_data,
							& mut uncompressed_data,
							flate2::Flush::Finish,
						).unwrap_or_else (
							|error|

							panic! (
								"ZLIB decompression failed: {:?}",
								error)

						)
					) {

						flate2::Status::Ok =>
							(),

						_ =>
							panic! (
								"ZLIB decompression failed"),

					}

					if decompress.total_out () as usize
						!= uncompressed_data.len () {

						panic! (
							"ZLIB decompressed size {} does not match {}",
							decompress.total_out (),
							uncompressed_data.len ());

					}

					Ok (Some (
						Cow::Owned (
							uncompressed_data
						)
					))

				},

				_ =>
					panic! (
						format! (
							"Unrecognised inline extent data compression {}",
							self.data ().compression)),

			}

		} else {

			Ok (None)

		}

	}

}

pub const BTRFS_EXTENT_DATA_INLINE_TYPE: u8 = 0;
pub const BTRFS_EXTENT_DATA_REGULAR_TYPE: u8 = 1;
pub const BTRFS_EXTENT_DATA_PREALLOC_TYPE: u8 = 2;

pub const BTRFS_EXTENT_DATA_NO_COMPRESSION: u8 = 0;
pub const BTRFS_EXTENT_DATA_ZLIB_COMPRESSION: u8 = 1;
pub const BTRFS_EXTENT_DATA_LZO_COMPRESSION: u8 = 2;

// ex: noet ts=4 filetype=rust
