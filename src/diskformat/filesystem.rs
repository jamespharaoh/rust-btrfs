use output::Output;

use super::*;
use super::BtrfsTree;

pub struct BtrfsFilesystem <'a> {
	devices: & 'a BtrfsDeviceSet <'a>,
	chunk_tree: BtrfsChunkTree <'a>,
	root_tree: BtrfsRootTree <'a>,
	fs_tree: BtrfsFsTree <'a>,
}

impl <'a> BtrfsFilesystem <'a> {

	pub fn open_try_backups (
		output: & Output,
		devices: & 'a BtrfsDeviceSet,
	) -> Result <BtrfsFilesystem <'a>, String> {

		Self::open (
			output,
			devices,
		).or_else (
			|error| {

			output_message! (
				output,
				"Error opening filesystem: {}",
				error);

			for backup_index in 0 .. 4 {

				output_message! (
					output,
					"Trying backup {}",
					backup_index);

				output_debug! (
					output,
					"{:?}",
					devices.superblock ().root_backups () [
						backup_index]);

				match Self::open_backup (
					output,
					devices,
					backup_index,
				) {

					Ok (filesystem) =>
						return Ok (filesystem),

					Err (error) =>
						output_message! (
							output,
							"Error opening filesystem backup {}: {}",
							backup_index,
							error),

				}

			}

			Err (
				format! (
					"Unable to open filesystem or any backups"))

		})

	}

	pub fn open (
		output: & Output,
		devices: & 'a BtrfsDeviceSet,
	) -> Result <BtrfsFilesystem <'a>, String> {

		// read chunk tree

		let output_job =
			output_job_start! (
				output,
			"Reading chunk tree");

		let chunk_tree =
			BtrfsChunkTree::new (
				& devices,
			) ?;

		output_message! (
			output,
			"Chunk tree logical address: 0x{:x}",
			devices.superblock ().chunk_tree_logical_address ());

		let physical_address =
			chunk_tree.logical_to_physical_address (
				devices.superblock ().chunk_tree_logical_address (),
			).unwrap ();

		output_message! (
			output,
			"Chunk tree physical address: {}/0x{:x}",
			physical_address.device_id (),
			physical_address.offset ());

		output_job.complete ();

		// read root tree

		let output_job =
			output_job_start! (
				output,
				"Reading root tree");

		let root_tree =
			BtrfsRootTree::read_logical_address (
				output,
				& devices,
				& chunk_tree,
				devices.superblock ().root_tree_logical_address (),
			) ?;

		output_job.complete ();

		// read fs tree

		let fs_tree = {

			let output_job =
				output_job_start! (
					output,
					"Reading fs tree");

			let fs_tree_root_item =
				root_tree.fs_tree_root_item ().ok_or (
					"Can't find fs tree",
				) ?;

			let fs_tree =
				BtrfsFsTree::read_logical_address (
					output,
					& devices,
					& chunk_tree,
					fs_tree_root_item.root_node_block_number (),
				) ?;

			output_job.complete ();

			fs_tree

		};

		// return

		Ok (BtrfsFilesystem {
			devices: devices,
			chunk_tree: chunk_tree,
			root_tree: root_tree,
			fs_tree: fs_tree,
		})

	}

	pub fn open_backup (
		output: & Output,
		devices: & 'a BtrfsDeviceSet,
		backup_index: usize,
	) -> Result <BtrfsFilesystem <'a>, String> {

		assert! (backup_index < 4);

		let root_backup =
			devices.superblock ().root_backups () [
				backup_index];

		// read chunk tree

		let output_job =
			output_job_start! (
				output,
				"Reading chunk tree");

		let chunk_tree =
			BtrfsChunkTree::new (
				& devices,
			) ?;

		output_message! (
			output,
			"Chunk tree logical address: 0x{:x}",
			devices.superblock ().chunk_tree_logical_address ());

		let physical_address =
			chunk_tree.logical_to_physical_address (
				devices.superblock ().chunk_tree_logical_address (),
			).unwrap ();

		output_message! (
			output,
			"Chunk tree physical address: {}/0x{:x}",
			physical_address.device_id (),
			physical_address.offset ());

		output_job.complete ();

		// read root tree

		let output_job =
			output_job_start! (
				output,
				"Reading root tree");

		let root_tree =
			BtrfsRootTree::read_logical_address (
				output,
				& devices,
				& chunk_tree,
				root_backup.tree_root,
			) ?;

		output_job.complete ();

		// read fs tree

		let output_job =
			output_job_start! (
				output,
				"Reading fs tree");

		let fs_tree =
			BtrfsFsTree::read_logical_address (
				output,
				& devices,
				& chunk_tree,
				root_backup.fs_root,
			) ?;

		output_job.complete ();

		// return

		Ok (BtrfsFilesystem {
			devices: devices,
			chunk_tree: chunk_tree,
			root_tree: root_tree,
			fs_tree: fs_tree,
		})

	}

	pub fn file_tree (
		& 'a self,
		output: & Output,
		tree_id: u64,
	) -> Result <BtrfsFileTree <'a>, String> {

		let output_job =
			output_job_start! (
				output,
				"Reading file tree {}",
				tree_id);

		let root_item =
			self.root_item (
				tree_id,
			).ok_or (

				format! (
					"No such tree: {}",
					tree_id)

			) ?;

		let file_tree =
			BtrfsFileTree::read_logical_address (
				output,
				& self.devices,
				& self.chunk_tree,
				root_item.root_node_block_number (),
			) ?;

		output_job.complete ();

		Ok (file_tree)

	}

	pub fn superblock (& 'a self) -> BtrfsSuperblock <'a> {
		self.devices.superblock ()
	}

	pub fn devices (& 'a self) -> & 'a BtrfsDeviceSet <'a> {
		self.devices
	}

	pub fn chunk_tree (& 'a self) -> & 'a BtrfsChunkTree <'a> {
		& self.chunk_tree
	}

	pub fn root_tree (& 'a self) -> & 'a BtrfsRootTree <'a> {
		& self.root_tree
	}

	pub fn fs_tree (& 'a self) -> & 'a BtrfsFsTree <'a> {
		& self.fs_tree
	}

	pub fn default_root_dir_item (
		& 'a self,
	) -> Option <& 'a BtrfsDirItem <'a>> {
		self.root_tree.default_root_dir_item ()
	}

	pub fn default_root_inode_item (
		& 'a self,
	) -> Option <& 'a BtrfsInodeItem <'a>> {
		self.root_tree.default_root_inode_item ()
	}

	pub fn default_subvolume_root_item (
		& 'a self,
	) -> Option <& 'a BtrfsRootItem <'a>> {
		self.root_tree.default_subvolume_root_item ()
	}

	pub fn root_item (
		& 'a self,
		tree_id: u64,
	) -> Option <& 'a BtrfsRootItem <'a>> {

		self.root_tree.root_item (
			tree_id,
		)

	}

}

// ex: noet ts=4 filetype=rust
