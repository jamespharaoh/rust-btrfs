use std::hash::Hash;
use std::hash::Hasher;

#[ derive (Copy) ]
pub struct BtrfsSuperblockSystemChunksData {
	pub data: [u8; 0x800],
}

impl Clone for BtrfsSuperblockSystemChunksData {

	fn clone (& self) -> BtrfsSuperblockSystemChunksData {
		* self
	}

}

impl Eq for BtrfsSuperblockSystemChunksData {
}

impl Hash for BtrfsSuperblockSystemChunksData {

	fn hash <State: Hasher> (
		& self,
		state: & mut State,
	) {

		(& self.data [..]).hash (
			state);

    }

}

impl PartialEq for BtrfsSuperblockSystemChunksData {

	fn eq (
		& self,
		other: & BtrfsSuperblockSystemChunksData,
	) -> bool {

		& self.data [..] == & other.data [..]

	}

}

// ex: noet ts=4 filetype=rust
