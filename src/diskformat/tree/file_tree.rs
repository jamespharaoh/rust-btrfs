use std::collections::BTreeMap;

use super::super::*;

pub struct BtrfsFileTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsFileTree <'a> {

	pub fn dir_items (
		& 'a self,
		object_id: u64,
	) -> Vec <& 'a BtrfsDirItem <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_DIR_ITEM_TYPE
			&& item.object_id () == object_id

		).map (
			|dir_item|

			leaf_item_destructure! (
				dir_item,
				DirItem,
			).unwrap ()

		).collect ()

	}

}

impl <'a> BtrfsTree <'a> for BtrfsFileTree <'a> {

	fn new (
		tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
	) -> BtrfsFileTree {

		BtrfsFileTree {
			tree_items: tree_items,
		}

	}

	fn tree_items (
		& 'a self,
	) -> & 'a BTreeMap <BtrfsKey, BtrfsLeafItem <'a>> {
		& self.tree_items
	}

}

// ex: noet ts=4 filetype=rust
