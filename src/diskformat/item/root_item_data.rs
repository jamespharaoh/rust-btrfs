use std::fmt::Debug;
use std::fmt::DebugStruct;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

use super::super::*;

#[ repr (C, packed) ]
#[ derive (Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct BtrfsRootItemData {
	pub inode_item: BtrfsInodeItemData,
	pub expected_generation: u64,
	pub root_object_id: u64,
	pub root_node_block_number: u64,
	pub byte_limit: u64,
	pub bytes_used: u64,
	pub last_snapshot_generation: u64,
	pub flags: u64,
	pub num_references: u32,
	pub drop_progress: BtrfsKey,
	pub drop_level: u8,
	pub tree_level: u8,
	pub generation_v2: u64,
	pub subvolume_uuid: BtrfsUuid,
	pub parent_uuid: BtrfsUuid,
	pub received_uuid: BtrfsUuid,
	pub changed_transaction_id: u64,
	pub created_transaction_id: u64,
	pub sent_transaction_id: u64,
	pub received_transaction_id: u64,
	pub changed_time: BtrfsTimestamp,
	pub created_time: BtrfsTimestamp,
	pub sent_time: BtrfsTimestamp,
	pub received_time: BtrfsTimestamp,
	pub reserved: [u64; 0x8],
}

impl BtrfsRootItemData {

	pub fn debug_struct (
		& self,
		debug_struct: & mut DebugStruct,
	) {

		debug_struct.field (
			"inode_item",
			& self.inode_item);

		debug_struct.field (
			"expected_generation",
			& self.expected_generation);

		debug_struct.field (
			"root_object_id",
			& self.root_object_id);

		debug_struct.field (
			"root_node_block_number",
			& self.root_node_block_number);

		debug_struct.field (
			"byte_limit",
			& self.byte_limit);

		debug_struct.field (
			"bytes_used",
			& self.bytes_used);

		debug_struct.field (
			"last_snapshot_generation",
			& self.last_snapshot_generation);

		debug_struct.field (
			"flags",
			& self.flags);

		debug_struct.field (
			"num_references",
			& self.num_references);

		debug_struct.field (
			"drop_progress",
			& NakedString::from (
				self.drop_progress.to_string ()));

		debug_struct.field (
			"drop_level",
			& self.drop_level);

		debug_struct.field (
			"tree_level",
			& self.tree_level);

		debug_struct.field (
			"generation_v2",
			& self.generation_v2);

		debug_struct.field (
			"subvolume_uuid",
			& NakedString::from (
				self.subvolume_uuid.to_string ()));

		debug_struct.field (
			"parent_uuid",
			& NakedString::from (
				self.parent_uuid.to_string ()));

		debug_struct.field (
			"received_uuid",
			& NakedString::from (
				self.received_uuid.to_string ()));

		debug_struct.field (
			"changed_transaction_id",
			& self.changed_transaction_id);

		debug_struct.field (
			"created_transaction_id",
			& self.created_transaction_id);

		debug_struct.field (
			"sent_transaction_id",
			& self.sent_transaction_id);

		debug_struct.field (
			"received_transaction_id",
			& self.received_transaction_id);

		debug_struct.field (
			"changed_time",
			& NakedString::from (
				self.changed_time.to_string ()));

		debug_struct.field (
			"created_time",
			& NakedString::from (
				self.created_time.to_string ()));

		debug_struct.field (
			"sent_time",
			& NakedString::from (
				self.sent_time.to_string ()));

		debug_struct.field (
			"received_time",
			& NakedString::from (
				self.received_time.to_string ()));

	}

}

impl Debug for BtrfsRootItemData {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsRootItemData");

		self.debug_struct (
			& mut debug_struct);

		debug_struct.finish ()

	}

}

#[ cfg (test) ]
mod tests {

	use std::mem;

	use super::*;

	#[ test ]
	fn test_size () {
		assert! (mem::size_of::<BtrfsRootItemData> () == 0x1b7);
	}

}

// ex: noet ts=4 filetype=rust
