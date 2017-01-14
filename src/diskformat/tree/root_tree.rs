use std::collections::BTreeMap;
use std::collections::HashMap;

use diskformat::*;

pub struct BtrfsRootTree {
}

impl BtrfsRootTree {

	pub fn new (
		devices: & BtrfsDeviceMap,
		superblock: & BtrfsSuperblock,
		chunk_tree: & BtrfsChunkTree,
	) -> Result <BtrfsRootTree, String> {

		let mut tree_items: HashMap <BtrfsKey, BtrfsLeafItem> =
			HashMap::new ();

		let mut errors: Vec <String> =
			Vec::new ();

		BtrfsTree::read_tree_recurse (
			devices,
			superblock,
			chunk_tree,
			superblock.root_tree_logical_address (),
			& mut tree_items,
			& mut errors,
		);

		for error in errors {

			println! (
				"Error reading root tree: {}",
				error);

		}

		for tree_item in tree_items.values () {

			println! (
				"Root tree item: {:?}",
				tree_item);

		}

		Ok (
			BtrfsRootTree {
			}
		)

	}

}

// ex: noet ts=4 filetype=rust
