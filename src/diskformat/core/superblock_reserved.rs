use std::hash::Hash;
use std::hash::Hasher;

#[ derive (Copy) ]
pub struct BtrfsSuperblockReserved {
	data: [u8; 0xf0],
}

impl Clone for BtrfsSuperblockReserved {

	fn clone (& self) -> BtrfsSuperblockReserved {
		* self
	}

}

impl Eq for BtrfsSuperblockReserved {
}

impl Hash for BtrfsSuperblockReserved {

	fn hash <State: Hasher> (
		& self,
		state: & mut State,
	) {

		(& self.data [..]).hash (
			state);

    }

}

impl PartialEq for BtrfsSuperblockReserved {

	fn eq (
		& self,
		other: & Self,
	) -> bool {

		& self.data [..] == & other.data [..]

	}

}

// ex: noet ts=4 filetype=rust
