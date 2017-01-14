use std::mem;

use diskformat::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInodeItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInodeItemData {
	generation: u64,
	transaction_id: u64,
	st_size: u64,
	st_blocks: u64,
	block_group: u64,
	st_nlink: u32,
	st_uid: u32,
	st_gid: u32,
	st_mode: u32,
	st_rdev: u64,
	flags: u64,
	sequence: u64,
	reserved: [u8; 0x20],
	st_atime: [u8; 0xc],
	st_ctime: [u8; 0xc],
	st_mtime: [u8; 0xc],
	otime: [u8; 0xc],
}

impl <'a> BtrfsInodeItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsInodeItem <'a>, String> {

		// sanity check

		if data_bytes.len () != mem::size_of::<BtrfsInodeItemData> () {

			return Err (
				format! (
					"Must be at exactly 0x{:x} bytes",
					mem::size_of::<BtrfsInodeItemData> ()));

		}

		// create inode item

		Ok (
			BtrfsInodeItem {
				header: header,
				data_bytes: data_bytes,
			}
		)

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

	pub fn data (& self) -> & BtrfsInodeItemData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsInodeItemData
			)
		}

	}

	pub fn object_id (& self) -> u64 {
		self.header.object_id ()
	}

	pub fn key (& self) -> BtrfsKey {
		self.header.key ()
	}

	pub fn generation (& self) -> u64 {
		self.data ().generation
	}

	pub fn transaction_id (& self) -> u64 {
		self.data ().transaction_id
	}

	pub fn st_size (& self) -> u64 {
		self.data ().st_size
	}

	pub fn st_blocks (& self) -> u64 {
		self.data ().st_blocks
	}

	pub fn block_group (& self) -> u64 {
		self.data ().block_group
	}

	pub fn st_nlink (& self) -> u32 {
		self.data ().st_nlink
	}

	pub fn st_uid (& self) -> u32 {
		self.data ().st_uid
	}

	pub fn st_gid (& self) -> u32 {
		self.data ().st_gid
	}

	pub fn st_mode (& self) -> u32 {
		self.data ().st_mode
	}

	pub fn st_rdev (& self) -> u64 {
		self.data ().st_rdev
	}

	pub fn flags (& self) -> u64 {
		self.data ().flags
	}

	pub fn sequence (& self) -> u64 {
		self.data ().sequence
	}

	pub fn st_atime (& self) -> [u8; 0xc] {
		self.data ().st_atime
	}

	pub fn st_ctime (& self) -> [u8; 0xc] {
		self.data ().st_ctime
	}

	pub fn st_mtime (& self) -> [u8; 0xc] {
		self.data ().st_mtime
	}

	pub fn otime (& self) -> [u8; 0xc] {
		self.data ().otime
	}

}

// ex: noet ts=4 filetype=rust
