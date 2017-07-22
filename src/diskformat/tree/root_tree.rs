use std::collections::BTreeMap;
use std::collections::btree_map::Iter as BTreeMapIter;

use diskformat::*;

pub struct BtrfsRootTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsRootTree <'a> {

	pub fn new (
		devices: & 'a BtrfsDeviceSet,
		chunk_tree: & 'a BtrfsChunkTree,
	) -> Result <BtrfsRootTree <'a>, String> {

		let (root_tree, errors) =
			Self::new_tolerant (
				devices,
				chunk_tree,
			);

		if errors.is_empty () {
			Ok (root_tree)
		} else {
			Err (errors.into_iter ().next ().unwrap ())
		}

	}

	pub fn new_tolerant (
		devices: & 'a BtrfsDeviceSet,
		chunk_tree: & 'a BtrfsChunkTree,
	) -> (BtrfsRootTree <'a>, Vec <String>) {

		let mut tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem> =
			BTreeMap::new ();

		let mut errors: Vec <String> =
			Vec::new ();

		BtrfsTree::read_tree_recurse_logical_address (
			devices,
			chunk_tree,
			devices.superblock ().root_tree_logical_address (),
			& mut tree_items,
			& mut errors,
		);

		(
			BtrfsRootTree {
				tree_items: tree_items,
			},
			errors,
		)

	}

	pub fn iter (
		& self,
	) -> BTreeMapIter <BtrfsKey, BtrfsLeafItem <'a>> {

		self.tree_items.iter ()

	}

}

// ex: noet ts=4 filetype=rust
