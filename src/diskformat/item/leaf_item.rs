use diskformat::*;

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
	Unknown (BtrfsUnknownItem <'a>),
}

impl <'a> BtrfsLeafItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> BtrfsLeafItem <'a> {

		match header.key ().item_type () {

			BTRFS_INODE_ITEM_TYPE =>
				BtrfsInodeItem::from_bytes (
					header,
					data_bytes,
				).map (
					|inode_item|

					BtrfsLeafItem::InodeItem (
						inode_item)

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

			BTRFS_DIR_INDEX_TYPE =>
				BtrfsDirIndex::from_bytes (
					header,
					data_bytes,
				).map (
					|dir_index|

					BtrfsLeafItem::DirIndex (
						dir_index)

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

			BTRFS_CHUNK_ITEM_TYPE =>
				BtrfsChunkItem::from_bytes (
					header,
					data_bytes,
				).map (
					|chunk_item|

					BtrfsLeafItem::ChunkItem (
						chunk_item)

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

	pub fn header (& self) -> & BtrfsLeafItemHeader {

		match self {

			& BtrfsLeafItem::ChunkItem (ref chunk_item) =>
				chunk_item.header (),

			& BtrfsLeafItem::DirIndex (ref dir_index) =>
				dir_index.header (),

			& BtrfsLeafItem::DirItem (ref dir_item) =>
				dir_item.header (),

			& BtrfsLeafItem::ExtentData (ref extent_data) =>
				extent_data.header (),

			& BtrfsLeafItem::ExtentItem (ref extent_item) =>
				extent_item.header (),

			& BtrfsLeafItem::InodeItem (ref inode_item) =>
				inode_item.header (),

			& BtrfsLeafItem::Unknown (ref unknown_item) =>
				unknown_item.header (),

			& BtrfsLeafItem::Invalid (ref invalid_item) =>
				invalid_item.header (),

			& BtrfsLeafItem::RootItem (ref root_item) =>
				root_item.header (),

		}

	}

	pub fn key (& self) -> BtrfsKey {
		self.header ().key ()
	}

}

// ex: noet ts=4 filetype=rust
