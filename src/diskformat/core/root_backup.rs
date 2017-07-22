use std::fmt::Debug;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::mem;

use super::super::*;

#[ repr (C, packed) ]
#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
pub struct BtrfsRootBackup {
	tree_root: u64,
	tree_root_gen: u64,
	chunk_root: u64,
	chunk_root_gen: u64,
	extent_root: u64,
	extent_root_gen: u64,
	fs_root: u64,
	fs_root_gen: u64,
	dev_root: u64,
	dev_root_gen: u64,
	csum_root: u64,
	csum_root_gen: u64,
	total_bytes: u64,
	bytes_used: u64,
	num_devices: u64,
	unused_0: [u64; 4],
	tree_root_level: u8,
	chunk_root_level: u8,
	extent_root_level: u8,
	fs_root_level: u8,
	dev_root_level: u8,
	csum_root_level: u8,
	unused_1: [u8; 10],
}

impl Debug for BtrfsRootBackup {

	fn fmt (
		& self,
		formatter: & mut Formatter,
	) -> Result <(), FmtError> {

		let mut debug_struct =
			formatter.debug_struct (
				"BtrfsRootBackup");

		debug_struct.field (
			"tree_root",
			& NakedString::from (
				format! (
					"0x{:x}",
					self.tree_root)));

		debug_struct.field (
			"tree_root_gen",
			& self.tree_root_gen);

		debug_struct.field (
			"chunk_root",
			& NakedString::from (
				format! (
					"0x{:x}",
					self.chunk_root)));

		debug_struct.field (
			"chunk_root_gen",
			& self.chunk_root_gen);

		debug_struct.field (
			"extent_root",
			& NakedString::from (
				format! (
					"0x{:x}",
					self.extent_root)));

		debug_struct.field (
			"extent_root_gen",
			& self.extent_root_gen);

		debug_struct.field (
			"fs_root",
			& NakedString::from (
				format! (
					"0x{:x}",
					self.fs_root)));

		debug_struct.field (
			"fs_root_gen",
			& self.fs_root_gen);

		debug_struct.field (
			"dev_root",
			& NakedString::from (
				format! (
					"0x{:x}",
					self.dev_root)));

		debug_struct.field (
			"dev_root_gen",
			& self.dev_root_gen);

		debug_struct.field (
			"csum_root",
			& NakedString::from (
				format! (
					"0x{:x}",
					self.csum_root)));

		debug_struct.field (
			"csum_root_gen",
			& self.csum_root_gen);

		debug_struct.field (
			"total_bytes",
			& self.total_bytes);

		debug_struct.field (
			"bytes_used",
			& self.bytes_used);

		debug_struct.field (
			"num_devices",
			& self.num_devices);

		debug_struct.field (
			"num_devices",
			& self.num_devices);

		debug_struct.field (
			"tree_root_level",
			& self.tree_root_level);

		debug_struct.field (
			"chunk_root_level",
			& self.chunk_root_level);

		debug_struct.field (
			"extent_root_level",
			& self.extent_root_level);

		debug_struct.field (
			"fs_root_level",
			& self.fs_root_level);

		debug_struct.field (
			"dev_root_level",
			& self.dev_root_level);

		debug_struct.field (
			"csum_root_level",
			& self.csum_root_level);

		debug_struct.finish ()

	}

}

#[ test ]
fn test_size () {
	assert! (mem::size_of::<BtrfsRootBackup> () == 0xa8);
}

// ex: noet ts=4 filetype=rust
