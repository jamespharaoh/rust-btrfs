use super::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsDevItem <'a> {
	data: & 'a BtrfsDevItemData,
}

impl <'a> BtrfsDevItem <'a> {

	pub fn new (
		data: & 'a BtrfsDevItemData,
	) -> BtrfsDevItem <'a> {

		BtrfsDevItem {
			data: data,
		}

	}

	pub fn device_id (& self) -> u64 {
		self.data.device_id
	}

}

// ex: noet ts=4 filetype=rust
