use diskformat::*;

#[ derive (Clone) ]
pub enum BtrfsNode <'a> {
	Internal (BtrfsInternalNode <'a>),
	Leaf (BtrfsLeafNode <'a>),
}

impl <'a> BtrfsNode <'a> {

	pub fn from_bytes (
		/*position: usize,*/
		bytes: & 'a [u8],
	) -> Result <BtrfsNode <'a>, String> {

		// verify checksum

		let calculated_checksum =
			BtrfsChecksum::for_bytes (
				& bytes [0x20 .. bytes.len ()]);

		let header = unsafe {
			& * (bytes.as_ptr () as * const BtrfsNodeHeader)
		};

		if header.checksum () != calculated_checksum {

			return Err (
				"Checksum mismatch".to_owned ());

		}

		// construct

		if header.level () == 0 {

			Ok (
				BtrfsNode::Leaf (
					BtrfsLeafNode::new (
						/*position,*/
						bytes,
					)
				)
			)

		} else {

			Ok (
				BtrfsNode::Internal (
					BtrfsInternalNode::new (
						/*position,*/
						bytes,
					)
				)
			)

		}

	}

}

// ex: noet ts=4 filetype=rust
