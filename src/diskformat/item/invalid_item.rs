use diskformat::*;

#[ derive (Clone, Debug) ]
pub struct BtrfsInvalidItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
	error: String,
}

impl <'a> BtrfsInvalidItem <'a> {

	pub fn new (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
		error: String,
	) -> BtrfsInvalidItem <'a> {

		BtrfsInvalidItem {
			header: header,
			data_bytes: data_bytes,
			error: error,
		}

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

// ex: noet ts=4 filetype=rust
