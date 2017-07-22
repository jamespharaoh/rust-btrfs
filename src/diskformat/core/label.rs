use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::str;
use std::str::Utf8Error;

pub const BTRFS_LABEL_BYTES: usize = 0x100;

#[ repr (C, packed) ]
#[ derive (Copy) ]
pub struct BtrfsLabel {
	data: [u8; BTRFS_LABEL_BYTES],
}

impl BtrfsLabel {

	pub fn len (
		& self,
	) -> usize {

		self.data.iter ().position (
			|byte| * byte == 0
		).unwrap_or (self.data.len ())

	}

	pub fn data (
		& self,
	) -> & [u8] {

		& self.data [0 .. self.len ()]

	}

	pub fn as_str (
		& self,
	) -> Result <& str, Utf8Error> {

		str::from_utf8 (
			self.data ())

	}

	pub fn to_string_lossy (
		& self,
	) -> Cow <str> {

		String::from_utf8_lossy (
			self.data ())

	}

}

impl Clone for BtrfsLabel {

	fn clone (& self) -> BtrfsLabel {
		* self
	}

}

impl Debug for BtrfsLabel {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_str (
			& self.to_string_lossy (),
		) ?;

		Ok (())

	}

}

impl Eq for BtrfsLabel {
}

impl Hash for BtrfsLabel {

	fn hash <State: Hasher> (
		& self,
		state: & mut State,
	) {

		(& self.data [..]).hash (
			state);

    }

}

impl PartialEq for BtrfsLabel {

	fn eq (
		& self,
		other: & BtrfsLabel,
	) -> bool {

		& self.data [..] == & other.data [..]

	}

}

// ex: noet ts=4 filetype=rust
