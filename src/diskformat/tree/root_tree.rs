use std::collections::BTreeMap;

use super::super::*;

pub struct BtrfsRootTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsRootTree <'a> {

	pub fn root_items (
		& 'a self,
	) -> Vec <& 'a BtrfsRootItem <'a>> {

		self.tree_items.values ().filter (
			|item| item.key ().item_type () == BTRFS_ROOT_ITEM_TYPE,
		).map (
			|item|

			match item {

				& BtrfsLeafItem::RootItem (ref item) =>
					item,

				_ =>
					panic! (),

			}

		).collect ()

	}

	pub fn root_item (
		& 'a self,
		tree_id: u64,
	) -> Option <& 'a BtrfsRootItem <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_ROOT_ITEM_TYPE
			&& item.object_id () == tree_id

		).map (
			|root_item|

			leaf_item_destructure! (
				root_item,
				RootItem,
			).unwrap ()

		).next ()

	}

	pub fn fs_tree_root_item (
		& 'a self,
	) -> Option <& 'a BtrfsRootItem <'a>> {

		self.get_by_key (
			BtrfsKey::new (
				BTRFS_FS_TREE_OBJECT_ID,
				BTRFS_ROOT_ITEM_TYPE,
				0,
			),
		).map (
			|item|

			match item  {

			& BtrfsLeafItem::RootItem (ref item) =>
				item,

			_ =>
				panic! (),

		})

	}

	pub fn subvolume_root_items (
		& 'a self,
	) -> Vec <& 'a BtrfsRootItem <'a>> {

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
			).unwrap ()

		).collect ()

	}

	pub fn subvolume_root_refs (
		& 'a self,
	) -> Vec <& 'a BtrfsRootRef <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_ROOT_REF_ITEM_TYPE

		).map (
			|root_ref|

			leaf_item_destructure! (
				root_ref,
				RootRef,
			).unwrap ()

		).collect ()

	}

	pub fn subvolume_root_backrefs (
		& 'a self,
	) -> Vec <& 'a BtrfsRootBackref <'a>> {

		self.tree_items.values ().filter (
			|item|

			item.item_type () == BTRFS_ROOT_BACKREF_ITEM_TYPE

		).map (
			|root_backref|

			leaf_item_destructure! (
				root_backref,
				RootBackref,
			).unwrap ()

		).collect ()

	}

	pub fn default_root_inode_item (
		& 'a self,
	) -> Option <& 'a BtrfsInodeItem <'a>> {

		self.get_by_key (
			BtrfsKey::new (
				BTRFS_DEFAULT_TREE_OBJECT_ID,
				BTRFS_INODE_ITEM_TYPE,
				0,
			),
		).map (
			|item|

			match item  {

			& BtrfsLeafItem::InodeItem (ref item) =>
				item,

			_ =>
				panic! (),

		})

	}

	pub fn default_root_dir_item (
		& 'a self,
	) -> Option <& 'a BtrfsDirItem <'a>> {

		self.get_by_key (
			BtrfsKey::new (
				BTRFS_DEFAULT_TREE_OBJECT_ID,
				BTRFS_DIR_ITEM_TYPE,
				btrfs_crc32_linux (
					b"default",
				) as u64,
			),
		).map (
			|item|

			match item  {

			& BtrfsLeafItem::DirItem (ref item) =>
				item,

			_ =>
				panic! (),

		})

	}

	pub fn default_subvolume_root_item (
		& 'a self,
	) -> Option <& 'a BtrfsRootItem <'a>> {

		self.default_root_dir_item ().and_then (
			|default_root_dir_item| {

			let default_root_dir_item_entry =
				default_root_dir_item.entries ().next ().unwrap ();

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
			).unwrap ()

		)

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
