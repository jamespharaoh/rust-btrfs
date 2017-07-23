use std::collections::BTreeMap;
use std::collections::btree_map::Iter as BTreeMapIter;

use super::super::*;

use output::*;

pub trait BtrfsTree <'a>: Sized {

	fn new (
		tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
	) -> Self;

	fn tree_items (
		& 'a self,
	) -> & 'a BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>;

	fn iter (
		& 'a self,
	) -> BTreeMapIter <BtrfsKey, BtrfsLeafItem <'a>> {
		self.tree_items ().iter ()
	}

	fn read_logical_address (
		output: & Output,
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		logical_address: u64,
	) -> Result <Self, String> {

		let (btrfs_tree, errors) =
			Self::read_tolerant_logical_address (
				output,
				devices,
				chunk_tree,
				logical_address,
			);

		if errors.is_empty () {
			Ok (btrfs_tree)
		} else {
			Err (errors.into_iter ().next ().unwrap ())
		}

	}

	fn read_physical_address (
		output: & Output,
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		physical_address: BtrfsPhysicalAddress,
	) -> Result <Self, String> {

		let (btrfs_tree, errors) =
			Self::read_tolerant_physical_address (
				output,
				devices,
				chunk_tree,
				physical_address,
			);

		if errors.is_empty () {
			Ok (btrfs_tree)
		} else {
			Err (errors.into_iter ().next ().unwrap ())
		}

	}

	fn read_tolerant_physical_address (
		output: & Output,
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		physical_address: BtrfsPhysicalAddress,
	) -> (Self, Vec <String>) {

		let mut tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem> =
			BTreeMap::new ();

		let mut errors: Vec <String> =
			Vec::new ();

		btrfs_read_tree_recurse_physical_address (
			output,
			devices,
			chunk_tree,
			physical_address,
			& mut tree_items,
			& mut errors,
		);

		(
			Self::new (tree_items),
			errors,
		)

	}

	fn read_tolerant_logical_address (
		output: & Output,
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		logical_address: u64,
	) -> (Self, Vec <String>) {

		let mut tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem> =
			BTreeMap::new ();

		let mut errors: Vec <String> =
			Vec::new ();

		btrfs_read_tree_recurse_logical_address (
			output,
			devices,
			chunk_tree,
			logical_address,
			& mut tree_items,
			& mut errors,
		);

		(
			Self::new (tree_items),
			errors,
		)

	}

	fn get_by_key (
		& 'a self,
		key: BtrfsKey,
	) -> Option <& 'a BtrfsLeafItem <'a>> {

		self.tree_items ().get (
			& key,
		)

	}

	fn get_by_object_id (
		& 'a self,
		object_id: u64,
	) -> Vec <& 'a BtrfsLeafItem <'a>> {

		self.tree_items ().range (
			BtrfsKey::new (object_id, 0, 0)
		..
			BtrfsKey::new (object_id + 1, 0, 0)
		).map (
			|(_key, value)| value,
		).collect ()

	}

	fn get_by_item_type (
		& 'a self,
		item_type: u8,
	) -> Vec <& 'a BtrfsLeafItem <'a>> {

		self.tree_items ().values ().filter (
			|item| item.key ().item_type () == item_type,
		).collect ()

	}

	fn get_root_items_by_object_id (
		& 'a self,
		object_id: u64,
	) -> Vec <& 'a BtrfsRootItem <'a>> {

		self.tree_items ().values ().filter (
			|item|

			item.item_type () == BTRFS_ROOT_ITEM_TYPE
			&& item.object_id () == object_id

		).map (
			|item|

			item.as_root_item ().unwrap ()

		).collect ()

	}

	fn dir_items (
		& 'a self,
	) -> Vec <& 'a BtrfsDirItem <'a>> {

		self.tree_items ().values ().filter (
			|item|

			item.item_type () == BTRFS_DIR_ITEM_TYPE

		).map (|item| match item  {

			& BtrfsLeafItem::DirItem (ref item) =>
				item,

			_ =>
				panic! (),

		}).collect ()

	}

}

// ex: noet ts=4 filetype=rust
