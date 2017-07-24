use super::super::prelude::*;

pub struct BtrfsExtentTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsExtentTree <'a> {

}

impl <'a> BtrfsTree <'a> for BtrfsExtentTree <'a> {

	fn new (
		tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
	) -> BtrfsExtentTree {

		BtrfsExtentTree {
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
