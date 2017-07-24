use super::super::prelude::*;

pub struct BtrfsFilesystemTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsFilesystemTree <'a> {

	tree_item_accessor! (
		dir_index,
		BtrfsDirIndex,
		BTRFS_DIR_INDEX_TYPE,
		DirIndex,
	);

	tree_item_range_accessor! (
		dir_indexes,
		BtrfsDirIndex,
		BTRFS_DIR_INDEX_TYPE,
		DirIndex,
	);

	tree_item_accessor! (
		dir_item,
		BtrfsDirItem,
		BTRFS_DIR_ITEM_TYPE,
		DirItem,
	);

	pub fn dir_item_entry (
		& 'a self,
		object_id: u64,
		name: & [u8],
	) -> Option <BtrfsDirItemEntry <'a>> {

		self.dir_item (
			object_id,
			btrfs_crc32_linux (name) as u64,
		).and_then (
			|dir_item|

			dir_item.entries ().find (
				|dir_item_entry|

				dir_item_entry.name () == name

			)

		)

	}

	tree_item_range_accessor! (
		dir_items,
		BtrfsDirItem,
		BTRFS_DIR_ITEM_TYPE,
		DirItem,
	);

	tree_item_range_accessor! (
		extent_datas,
		BtrfsExtentData,
		BTRFS_EXTENT_DATA_TYPE,
		ExtentData,
	);

	tree_item_accessor! (
		inode_item,
		BtrfsInodeItem,
		BTRFS_INODE_ITEM_TYPE,
		InodeItem,
		0,
	);

	tree_item_range_accessor! (
		inode_refs,
		BtrfsInodeRef,
		BTRFS_INODE_REF_TYPE,
		InodeRef,
	);

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
