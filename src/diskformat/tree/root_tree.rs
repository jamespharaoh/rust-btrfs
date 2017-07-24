use std::collections::BTreeMap;

use super::super::*;

pub struct BtrfsRootTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsRootTree <'a> {

	tree_item_accessor! (
		dir_item,
		BtrfsDirItem,
		BTRFS_DIR_ITEM_TYPE,
		DirItem,
	);

	pub fn default_root_dir_item_entry (
		& 'a self,
	) -> Option <BtrfsDirItemEntry <'a>> {

		self.dir_item (
			BTRFS_DEFAULT_TREE_OBJECT_ID,
			btrfs_crc32_linux (b"default") as u64,
		).and_then (
			|dir_item|

			dir_item.entries ().next ()

		)

	}

	pub fn default_root_inode_item (
		& 'a self,
	) -> Option <BtrfsInodeItem <'a>> {

		self.inode_item (
			BTRFS_DEFAULT_TREE_OBJECT_ID,
			0,
		)

	}

	pub fn default_subvolume_root_item (
		& 'a self,
	) -> Option <BtrfsRootItem <'a>> {

		self.default_root_dir_item_entry ().and_then (
			|default_root_dir_item_entry| {

			self.get_by_key (
				BtrfsKey::new (
					default_root_dir_item_entry.child_object_id (),
					BTRFS_ROOT_ITEM_TYPE,
					0,
				),
			)

		}).map (
			|root_item|

			leaf_item_destructure! (
				root_item,
				RootItem,
			).unwrap ().clone ()

		)

	}

	pub fn fs_tree_root_item (
		& 'a self,
	) -> Option <BtrfsRootItem <'a>> {

		self.root_item (
			BTRFS_FS_TREE_OBJECT_ID,
		)

	}

	tree_item_accessor! (
		inode_item,
		BtrfsInodeItem,
		BTRFS_INODE_ITEM_TYPE,
		InodeItem,
	);

	pub fn subvolume_root_items (
		& 'a self,
	) -> Vec <BtrfsRootItem <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_ROOT_ITEM_TYPE
			&& (
				item.object_id () as i64 >= 128
				|| item.object_id () as i64 <= -128
				|| item.object_id () == 5
			)

		).map (
			|root_item|

			leaf_item_destructure! (
				root_item,
				RootItem,
			).unwrap ().clone ()

		).collect ()

	}

	pub fn subvolume_root_refs (
		& 'a self,
	) -> Vec <BtrfsRootRef <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_ROOT_REF_ITEM_TYPE

		).map (
			|root_ref|

			leaf_item_destructure! (
				root_ref,
				RootRef,
			).unwrap ().clone ()

		).collect ()

	}

	tree_item_accessor! (
		root_item,
		BtrfsRootItem,
		BTRFS_ROOT_ITEM_TYPE,
		RootItem,
		0,
	);

	tree_item_range_accessor! (
		root_items,
		BtrfsRootItem,
		BTRFS_ROOT_ITEM_TYPE,
		RootItem,
	);

	pub fn subvolume_root_backrefs (
		& 'a self,
	) -> Vec <BtrfsRootBackref <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_ROOT_BACKREF_ITEM_TYPE

		).map (
			|root_backref|

			leaf_item_destructure! (
				root_backref,
				RootBackref,
			).unwrap ().clone ()

		).collect ()

	}

}

impl <'a> BtrfsTree <'a> for BtrfsRootTree <'a> {

	fn new (
		tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
	) -> BtrfsRootTree {

		BtrfsRootTree {
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
