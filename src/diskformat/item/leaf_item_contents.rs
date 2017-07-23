use super::super::*;

pub trait BtrfsLeafItemContents <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader;

	fn item_type (& self) -> u8 {
		self.header ().item_type ()
	}

	fn key (& self) -> BtrfsKey {
		self.header ().key ()
	}

	fn object_id (& self) -> u64 {
		self.header ().object_id ()
	}

	fn offset (& self) -> u64 {
		self.header ().offset ()
	}

}

// ex: noet ts=4 filetype=rust
