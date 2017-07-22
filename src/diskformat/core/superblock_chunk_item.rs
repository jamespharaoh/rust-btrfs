use super::super::*;

pub struct BtrfsSuperblockChunkItem <'a> {
	key: & 'a BtrfsKey,
	data: & 'a BtrfsChunkItemData,
}

impl <'a> BtrfsSuperblockChunkItem <'a> {

	pub fn new (
		key: & 'a BtrfsKey,
		data: & 'a BtrfsChunkItemData,
	) -> Result <BtrfsSuperblockChunkItem <'a>, String> {

		if key.item_type () != BTRFS_CHUNK_ITEM_TYPE {

			return Err (
				format! (
					"Invalid key type for chunk item: 0x{:02x}",
					key.item_type ()));

		}

		Ok (
			BtrfsSuperblockChunkItem {
				key: key,
				data: data,
			}
		)

	}

	pub fn key (& self) -> & BtrfsKey {
		self.key
	}

	pub fn data (& self) -> & BtrfsChunkItemData {
		self.data
	}

	pub fn stripes (& self) -> & [BtrfsChunkItemStripeData] {
		self.data.stripes ()
	}

}

// ex: noet ts=4 filetype=rust
