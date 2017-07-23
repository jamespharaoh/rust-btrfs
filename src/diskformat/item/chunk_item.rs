use std::mem;

use super::super::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsChunkItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

impl <'a> BtrfsChunkItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsChunkItem <'a>, String> {

		// sanity check

		if data_bytes.len () < mem::size_of::<BtrfsChunkItemData> () {

			return Err (
				format! (
					"Must be at least 0x{:x} bytes",
					mem::size_of::<BtrfsInodeItemData> ()));

		}

		// TODO check stripes

		// create chunk item

		Ok (
			BtrfsChunkItem {
				header: header,
				data_bytes: data_bytes,
			}
		)

	}

	pub fn data (& self) -> & BtrfsChunkItemData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsChunkItemData
			)
		}

	}

	pub fn stripes (& self) -> & [BtrfsChunkItemStripeData] {
		self.data ().stripes ()
	}

}

impl <'a> BtrfsLeafItemContents <'a> for BtrfsChunkItem <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

// ex: noet ts=4 filetype=rust
