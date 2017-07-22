use std::hash::Hash;
use std::hash::Hasher;

#[ derive (Copy) ]
pub struct BtrfsSuperblockUnused {
	data: [u8; 0x235],
}

impl Clone for BtrfsSuperblockUnused {

	fn clone (& self) -> BtrfsSuperblockUnused {
		* self
	}

}

impl Eq for BtrfsSuperblockUnused {
}

impl Hash for BtrfsSuperblockUnused {

	fn hash <State: Hasher> (
		& self,
		state: & mut State,
	) {

		(& self.data [..]).hash (
			state);

    }

}

impl PartialEq for BtrfsSuperblockUnused {

	fn eq (
		& self,
		other: & BtrfsSuperblockUnused,
	) -> bool {

		& self.data [..] == & other.data [..]

	}

}

// ex: noet ts=4 filetype=rust
