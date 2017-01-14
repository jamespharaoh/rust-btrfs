use diskformat::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInternalItem {
	key: BtrfsKey,
	block_number: u64,
	generation: u64,
}

impl BtrfsInternalItem {

	pub fn key (& self) -> BtrfsKey {
		self.key
	}

	pub fn block_number (& self) -> u64 {
		self.block_number
	}

	pub fn generation (& self) -> u64 {
		self.generation
	}

}

// ex: noet ts=4 filetype=rust
