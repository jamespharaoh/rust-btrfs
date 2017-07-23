use std::collections::BTreeMap;

use super::super::*;

pub struct BtrfsFilesystemTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsFilesystemTree <'a> {

	pub fn dir_items (
		& 'a self,
		directory_id: u64,
	) -> Vec <BtrfsDirItem <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_DIR_ITEM_TYPE
			&& item.object_id () == directory_id

		).map (
			|dir_item|

			leaf_item_destructure! (
				dir_item,
				DirItem,
			).unwrap ().clone ()

		).collect ()

	}

	pub fn dir_index (
		& 'a self,
		directory_id: u64,
		sequence: u64,
	) -> Option <BtrfsDirIndex <'a>> {

		self.tree_items.get (
			& BtrfsKey::new (
				directory_id,
				BTRFS_DIR_INDEX_TYPE,
				sequence),
		).map (
			|dir_index|

			leaf_item_destructure! (
				dir_index,
				DirIndex,
			).unwrap ().clone ()

		)

	}

	pub fn dir_indexes (
		& 'a self,
		directory_id: u64,
	) -> Vec <BtrfsDirIndex <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_DIR_INDEX_TYPE
			&& item.object_id () == directory_id

		).map (
			|dir_index|

			leaf_item_destructure! (
				dir_index,
				DirIndex,
			).unwrap ().clone ()

		).collect ()

	}

	pub fn inode_refs (
		& 'a self,
		inode_id: u64,
	) -> Vec <BtrfsInodeRef <'a>> {

		self.tree_items ().range (
			BtrfsKey::new (
				inode_id,
				BTRFS_INODE_REF_TYPE,
				0)
		..
			BtrfsKey::new (
				inode_id,
				BTRFS_INODE_REF_TYPE + 1,
				0)
		).map (
			|(_key, inode_ref)|

			leaf_item_destructure! (
				inode_ref,
				InodeRef,
			).unwrap ().clone ()

		).collect ()

	}

}

impl <'a> BtrfsTree <'a> for BtrfsFilesystemTree <'a> {

	fn new (
		tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
	) -> BtrfsFilesystemTree {

		BtrfsFilesystemTree {
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
