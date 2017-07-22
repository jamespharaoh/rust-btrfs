use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
pub struct BtrfsPhysicalAddress {
	device_id: u64,
	offset: u64,
}

impl BtrfsPhysicalAddress {

	pub fn new (
		device_id: u64,
		offset: u64,
	) -> BtrfsPhysicalAddress {

		BtrfsPhysicalAddress {
			device_id: device_id,
			offset: offset,
		}

	}

	pub fn device_id (& self) -> u64 {
		self.device_id
	}

	pub fn offset (& self) -> u64 {
		self.offset
	}

}

impl Debug for BtrfsPhysicalAddress {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsPhysicalAddress");

		debug_struct.field (
			"device_id",
			& self.device_id ());

		debug_struct.field (
			"offset",
			& format! (
				"0x{:x}",
				self.offset ()));

		debug_struct.finish ()

	}

}

impl Display for BtrfsPhysicalAddress {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		formatter.write_fmt (
			format_args! (
				"{}/0x{:x}",
				self.device_id (),
				self.offset ()))

	}

}

// ex: noet ts=4 filetype=rust
