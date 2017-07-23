use super::super::*;

#[ derive (Clone, Debug) ]
pub enum BtrfsLeafItem <'a> {
	ChunkItem (BtrfsChunkItem <'a>),
	DirIndex (BtrfsDirIndex <'a>),
	DirItem (BtrfsDirItem <'a>),
	ExtentData (BtrfsExtentData <'a>),
	ExtentItem (BtrfsExtentItem <'a>),
	InodeItem (BtrfsInodeItem <'a>),
	Invalid (BtrfsInvalidItem <'a>),
	RootItem (BtrfsRootItem <'a>),
	RootRef (BtrfsRootRef <'a>),
	Unknown (BtrfsUnknownItem <'a>),
}

impl <'a> BtrfsLeafItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> BtrfsLeafItem <'a> {

		match header.key ().item_type () {

			BTRFS_CHUNK_ITEM_TYPE =>
				BtrfsChunkItem::from_bytes (
					header,
					data_bytes,
				).map (
					|chunk_item|

					BtrfsLeafItem::ChunkItem (
						chunk_item)

				),

			BTRFS_DIR_INDEX_TYPE =>
				BtrfsDirIndex::from_bytes (
					header,
					data_bytes,
				).map (
					|dir_index|

					BtrfsLeafItem::DirIndex (
						dir_index)

				),

			BTRFS_DIR_ITEM_TYPE =>
				BtrfsDirItem::from_bytes (
					header,
					data_bytes,
				).map (
					|dir_item|

					BtrfsLeafItem::DirItem (
						dir_item)

				),

			BTRFS_EXTENT_DATA_TYPE =>
				BtrfsExtentData::from_bytes (
					header,
					data_bytes,
				).map (
					|extent_data|

					BtrfsLeafItem::ExtentData (
						extent_data)

				),

			BTRFS_EXTENT_ITEM_TYPE =>
				BtrfsExtentItem::from_bytes (
					header,
					data_bytes,
				).map (
					|extent_item|

					BtrfsLeafItem::ExtentItem (
						extent_item)

				),

			BTRFS_INODE_ITEM_TYPE =>
				BtrfsInodeItem::from_bytes (
					header,
					data_bytes,
				).map (
					|inode_item|

					BtrfsLeafItem::InodeItem (
						inode_item)

				),

			BTRFS_ROOT_ITEM_TYPE =>
				BtrfsRootItem::from_bytes (
					header,
					data_bytes,
				).map (
					|root_item|

					BtrfsLeafItem::RootItem (
						root_item)

				),

			BTRFS_ROOT_REF_ITEM_TYPE =>
				BtrfsRootRef::from_bytes (
					header,
					data_bytes,
				).map (
					|root_ref|

					BtrfsLeafItem::RootRef (
						root_ref)

				),

			_ =>
				Ok (
					BtrfsLeafItem::Unknown (
						BtrfsUnknownItem::new (
							header,
							data_bytes,
						)
					)
				),

		}.unwrap_or_else (
			|error|

			BtrfsLeafItem::Invalid (
				BtrfsInvalidItem::new (
					header,
					data_bytes,
					error,
				)
			)

		)

	}

	pub fn contents (
		& 'a self,
	) -> Box <& 'a BtrfsLeafItemContents <'a>> {

		match self {

			& BtrfsLeafItem::ChunkItem (ref chunk_item) =>
				Box::new (chunk_item),

			& BtrfsLeafItem::DirIndex (ref dir_index) =>
				Box::new (dir_index),

			& BtrfsLeafItem::DirItem (ref dir_item) =>
				Box::new (dir_item),

			& BtrfsLeafItem::ExtentData (ref extent_data) =>
				Box::new (extent_data),

			& BtrfsLeafItem::ExtentItem (ref extent_item) =>
				Box::new (extent_item),

			& BtrfsLeafItem::InodeItem (ref inode_item) =>
				Box::new (inode_item),

			& BtrfsLeafItem::Invalid (ref invalid_item) =>
				Box::new (invalid_item),

			& BtrfsLeafItem::RootItem (ref root_item) =>
				Box::new (root_item),

			& BtrfsLeafItem::RootRef (ref root_ref) =>
				Box::new (root_ref),

			& BtrfsLeafItem::Unknown (ref unknown_item) =>
				Box::new (unknown_item),

		}

	}

	pub fn header (& self) -> & BtrfsLeafItemHeader {
		self.contents ().header ()
	}

	pub fn key (& self) -> BtrfsKey {
		self.header ().key ()
	}

	pub fn object_id (& self) -> u64 {
		self.contents ().object_id ()
	}

	pub fn item_type (& self) -> u8 {
		self.contents ().item_type ()
	}

	pub fn offset (& self) -> u64 {
		self.contents ().offset ()
	}

	pub fn as_root_item (
		& 'a self,
	) -> Option <& 'a BtrfsRootItem <'a>> {

		match self {

			& BtrfsLeafItem::RootItem (ref item) =>
				Some (item),

			_ =>
				None,

		}

	}

}

// ex: noet ts=4 filetype=rust// ex: noet ts=4 filetype=rust
