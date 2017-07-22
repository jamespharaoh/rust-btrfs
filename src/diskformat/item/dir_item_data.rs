use std::fmt::Debug;
use std::fmt::DebugStruct;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsDirItemData {
	pub child_key: BtrfsKey,
	pub transaction_id: u64,
	pub data_size: u16,
	pub name_size: u16,
	pub child_type: u8,
}

impl BtrfsDirItemData {

	pub fn debug_struct (
		& self,
		debug_struct: & mut DebugStruct,
	) {

		debug_struct.field (
			"child_key",
			& NakedString::from (
				self.child_key.to_string ()));

		debug_struct.field (
			"transaction_id",
			& self.transaction_id);

		debug_struct.field (
			"data_size",
			& self.data_size);

		debug_struct.field (
			"name_size",
			& self.name_size);

		debug_struct.field (
			"child_type",
			& self.child_type);

	}

}

impl Debug for BtrfsDirItemData {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsDirItemData");

		self.debug_struct (
			& mut debug_struct);

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
