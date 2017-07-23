use std::fmt::Debug;
use std::fmt::DebugStruct;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInodeItemData {
	pub generation: u64,
	pub transaction_id: u64,
	pub st_size: u64,
	pub st_blocks: u64,
	pub block_group: u64,
	pub st_nlink: u32,
	pub st_uid: u32,
	pub st_gid: u32,
	pub st_mode: u32,
	pub st_rdev: u64,
	pub flags: u64,
	pub sequence: u64,
	pub reserved: [u8; 0x20],
	pub st_atime: BtrfsTimestamp,
	pub st_ctime: BtrfsTimestamp,
	pub st_mtime: BtrfsTimestamp,
	pub st_otime: BtrfsTimestamp,
}

impl BtrfsInodeItemData {

	pub fn debug_struct (
		& self,
		debug_struct: & mut DebugStruct,
	) {

		debug_struct.field (
			"generation",
			& self.generation);

		debug_struct.field (
			"transaction_id",
			& self.transaction_id);

		debug_struct.field (
			"st_size",
			& self.st_size);

		debug_struct.field (
			"st_blocks",
			& self.st_blocks);

		debug_struct.field (
			"block_group",
			& self.block_group);

		debug_struct.field (
			"st_nlink",
			& self.st_nlink);

		debug_struct.field (
			"st_uid",
			& self.st_uid);

		debug_struct.field (
			"st_gid",
			& self.st_gid);

		debug_struct.field (
			"st_mode",
			& NakedString::from (
				format! (
					"0o{:5o}",
					self.st_mode)));

		debug_struct.field (
			"st_rdev",
			& self.st_rdev);

		debug_struct.field (
			"flags",
			& self.flags);

		debug_struct.field (
			"sequence",
			& self.sequence);

		debug_struct.field (
			"st_atime",
			& NakedString::from (
				self.st_atime.to_string ()));

		debug_struct.field (
			"st_ctime",
			& NakedString::from (
				self.st_ctime.to_string ()));

		debug_struct.field (
			"st_mtime",
			& NakedString::from (
				self.st_mtime.to_string ()));

		debug_struct.field (
			"st_otime",
			& NakedString::from (
				self.st_otime.to_string ()));

	}

}

impl Debug for BtrfsInodeItemData {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsInodeItemData");

		self.debug_struct (
			& mut debug_struct);

		debug_struct.finish ()

	}

}

#[ cfg (test) ]
mod tests {

	use std::mem;

	use super::*;

	#[ test ]
	fn test_size () {
		assert! (mem::size_of::<BtrfsInodeItemData> () == 0xa0);
	}

}

// ex: noet ts=4 filetype=rust
