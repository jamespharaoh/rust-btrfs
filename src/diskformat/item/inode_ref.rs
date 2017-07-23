use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use super::super::*;

#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsInodeRef <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

leaf_item_composite_type_entries! (
	BtrfsInodeRef,
	BtrfsInodeRefEntries,
);

leaf_item_composite_type_implementation! (
	BtrfsInodeRef,
	BtrfsInodeRefData,
	BtrfsInodeRefEntries,
	BtrfsInodeRefIterator,
);

leaf_item_composite_type_iterator! (
	BtrfsInodeRef,
	BtrfsInodeRefData,
	BtrfsInodeRefEntry,
	BtrfsInodeRefIterator,
	name_length,
);

// ex: noet ts=4 filetype=rust
