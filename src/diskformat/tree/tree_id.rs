use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
pub struct BtrfsTreeId (u64);

impl BtrfsTreeId {

	pub fn new (object_id: u64) -> BtrfsTreeId {
		BtrfsTreeId (object_id)
	}

	pub fn object_id (& self) -> u64 {
		self.0
	}

	pub fn is_root (& self) -> bool {
		self.object_id () == BTRFS_ROOT_TREE_OBJECT_ID
	}

	pub fn to_string (& self) -> String {

		if let Some (tree_name) =
			BTRFS_TREE_NAMES.get (
				& self.object_id (),
			) {

			format! (
				"{} = {}",
				tree_name,
				self.object_id ())

		} else {

			format! (
				"{}",
				self.object_id ())

		}

	}

}

impl From <u64> for BtrfsTreeId {

	fn from (
		object_id: u64,
	) -> BtrfsTreeId {

		BtrfsTreeId::new (
			object_id)

	}

}

impl Debug for BtrfsTreeId {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		if let Some (tree_name) =
			BTRFS_TREE_NAMES.get (
				& self.object_id (),
			) {

			formatter.write_fmt (
				format_args! (
					"BtrfsTreeId ({} = {})",
					tree_name,
					self.object_id ()))

		} else {

			formatter.write_fmt (
				format_args! (
					"BtrfsTreeId ({})",
					self.object_id ()))

		}

	}

}

impl Display for BtrfsTreeId {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		if let Some (tree_name) =
			BTRFS_TREE_NAMES.get (
				& self.object_id (),
			) {

			formatter.write_fmt (
				format_args! (
					"{} = {}",
					tree_name,
					self.object_id ()))

		} else {

			formatter.write_fmt (
				format_args! (
					"{}",
					self.object_id ()))

		}

	}

}

impl From <BtrfsTreeId> for u64 {

	fn from (
		btrfs_tree_id: BtrfsTreeId,
	) -> u64 {

		let BtrfsTreeId (object_id) =
			btrfs_tree_id;

		object_id

	}

}

impl From <BtrfsTreeId> for String {

	fn from (
		btrfs_tree_id: BtrfsTreeId,
	) -> String {

		btrfs_tree_id.to_string ()

	}

}

lazy_static! {

	static ref BTRFS_TREE_NAMES: HashMap <u64, & 'static str> = {

		let mut hash_map =
			HashMap::new ();

		hash_map.insert (
			BTRFS_ROOT_TREE_OBJECT_ID,
			"ROOT");

		hash_map.insert (
			BTRFS_EXTENT_TREE_OBJECT_ID,
			"EXTENT");

		hash_map.insert (
			BTRFS_CHUNK_TREE_OBJECT_ID,
			"CHUNK");

		hash_map.insert (
			BTRFS_DEV_TREE_OBJECT_ID,
			"DEV");

		hash_map.insert (
			BTRFS_FS_TREE_OBJECT_ID,
			"FS");

		hash_map

	};

}

pub const BTRFS_ROOT_TREE_OBJECT_ID: u64 = 1;
pub const BTRFS_EXTENT_TREE_OBJECT_ID: u64 = 2;
pub const BTRFS_CHUNK_TREE_OBJECT_ID: u64 = 3;
pub const BTRFS_DEV_TREE_OBJECT_ID: u64 = 4;
pub const BTRFS_FS_TREE_OBJECT_ID: u64 = 5;
pub const BTRFS_DEFAULT_TREE_OBJECT_ID: u64 = 6;

pub const BTRFS_ROOT_TREE_ID: BtrfsTreeId = BtrfsTreeId (1);
pub const BTRFS_EXTENT_TREE_ID: BtrfsTreeId = BtrfsTreeId (2);
pub const BTRFS_CHUNK_TREE_ID: BtrfsTreeId = BtrfsTreeId (3);
pub const BTRFS_DEV_TREE_ID: BtrfsTreeId = BtrfsTreeId (4);
pub const BTRFS_FS_TREE_ID: BtrfsTreeId = BtrfsTreeId (5);
pub const BTRFS_DEFAULT_TREE_ID: BtrfsTreeId = BtrfsTreeId (6);

// ex: noet ts=4 filetype=rust
