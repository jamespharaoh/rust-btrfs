use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsKey {
	object_id: u64,
	item_type: u8,
	offset: u64,
}

impl BtrfsKey {

	pub fn new (
		object_id: u64,
		item_type: u8,
		offset: u64,
	) -> BtrfsKey {

		BtrfsKey {
			object_id: object_id,
			item_type: item_type,
			offset: offset,
		}

	}

	pub fn object_id (& self) -> u64 {
		self.object_id
	}

	pub fn item_type (& self) -> u8 {
		self.item_type
	}

	pub fn offset (& self) -> u64 {
		self.offset
	}

	pub fn to_string (& self) -> String {

		format! (
			"{}/{} @ 0x{:x}",
			self.object_id,
			self.item_type,
			self.offset)

	}

	pub fn to_string_decimal (& self) -> String {

		format! (
			"{}/{} @ {}",
			self.object_id,
			self.item_type,
			self.offset)

	}

	pub fn to_string_no_type (& self) -> String {

		format! (
			"{} @ 0x{:x}",
			self.object_id,
			self.offset)

	}

	pub fn to_string_no_type_decimal (& self) -> String {

		format! (
			"{} @ {}",
			self.object_id,
			self.offset)

	}

}

impl Debug for BtrfsKey {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"BtrfsKey ({}/{} @ 0x{:x})",
				self.object_id,
				self.item_type,
				self.offset))

	}

}

// ex: noet ts=4 filetype=rust
