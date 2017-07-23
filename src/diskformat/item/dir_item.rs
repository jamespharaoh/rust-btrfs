use super::super::prelude::*;

#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsDirItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

leaf_item_composite_type_entries! (
	BtrfsDirItem,
	BtrfsDirItemEntries,
);

leaf_item_composite_type_implementation! (
	BtrfsDirItem,
	BtrfsDirItemData,
	BtrfsDirItemEntries,
	BtrfsDirItemIterator,
);

leaf_item_composite_type_iterator! (
	BtrfsDirItem,
	BtrfsDirItemData,
	BtrfsDirItemEntry,
	BtrfsDirItemIterator,
	data_size,
	name_size,
);

// ex: noet ts=4 filetype=rust
