use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use diskformat::*;

#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsRootBackref <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

impl <'a> BtrfsRootBackref <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsRootBackref <'a>, String> {

		// sanity check

		if data_bytes.len () < mem::size_of::<BtrfsRootRefData> () {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsRootRefData> ()));

		}

		// create root backref

		let root_ref = BtrfsRootBackref {
			header: header,
			data_bytes: data_bytes,
		};

		// sanity check

		if data_bytes.len () != (
			mem::size_of::<BtrfsRootRefData> ()
			+ root_ref.name_length () as usize
		) {

			return Err (
				format! (
					"Must be at exactly 0x{:x} bytes",
					mem::size_of::<BtrfsRootRefData> ()
					+ root_ref.name_length () as usize));

		}

		// return

		Ok (root_ref)

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

	pub fn data (& self) -> & BtrfsRootRefData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsRootRefData
			)
		}

	}

	pub fn directory_id (& self) -> u64 {
		self.data ().directory_id
	}

	pub fn sequence (& self) -> u64 {
		self.data ().sequence
	}

	pub fn name_length (& self) -> u16 {
		self.data ().name_length
	}

	pub fn name (
		& 'a self,
	) -> & 'a [u8] {

		& self.data_bytes [
			mem::size_of::<BtrfsRootRefData> ()
		..
			mem::size_of::<BtrfsRootRefData> ()
			+ self.name_length () as usize
		]

	}

	pub fn name_to_string_lossy (
		& self,
	) -> Cow <str> {

		String::from_utf8_lossy (
			self.name ())

	}

}

impl <'a> BtrfsLeafItemContents <'a> for BtrfsRootBackref <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

impl <'a> Debug for BtrfsRootBackref <'a> {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsRootBackref");

		debug_struct.field (
			"key",
			& NakedString::from (
				self.key ().to_string_decimal ()));

		debug_struct.field (
			"directory_id",
			& self.directory_id ());

		debug_struct.field (
			"sequence",
			& self.sequence ());

		debug_struct.field (
			"name",
			& self.name_to_string_lossy ());

		debug_struct.finish ()

	}

}

// ex: noet ts=4 filetype=rust
