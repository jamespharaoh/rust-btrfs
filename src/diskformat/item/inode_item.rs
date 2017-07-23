use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use super::super::*;

#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInodeItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
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

	pub fn data (& self) -> & BtrfsInodeItemData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsInodeItemData
			)
		}

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

	pub fn st_atime (& self) -> BtrfsTimestamp {
		self.data ().st_atime
	}

	pub fn st_ctime (& self) -> BtrfsTimestamp {
		self.data ().st_ctime
	}

	pub fn st_mtime (& self) -> BtrfsTimestamp {
		self.data ().st_mtime
	}

	pub fn st_otime (& self) -> BtrfsTimestamp {
		self.data ().st_otime
	}

}

impl <'a> BtrfsLeafItemContents <'a> for BtrfsInodeItem <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

impl <'a> Debug for BtrfsInodeItem <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsInodeItem");

		self.header ().debug_struct (
			& mut debug_struct);

		self.data ().debug_struct (
			& mut debug_struct);

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
