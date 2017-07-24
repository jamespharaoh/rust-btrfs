use super::prelude::*;

pub struct BtrfsFilesystem <'a> {
	devices: & 'a BtrfsDeviceSet <'a>,
	chunk_tree: BtrfsChunkTree <'a>,
	filesystem_trees: HashMap <u64, BtrfsFilesystemTree <'a>>,
	root_tree: BtrfsRootTree <'a>,
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

		// read filesystem trees

		let filesystem_trees = {

			let output_job =
				output_job_start! (
					output,
					"Reading filesystem trees");

			let filesystem_trees =
				Self::read_filesystem_trees (
					output,
					& devices,
					& chunk_tree,
					& root_tree,
				) ?;

			output_job.complete ();

			filesystem_trees

		};

		// return

		Ok (BtrfsFilesystem {
			devices: devices,
			chunk_tree: chunk_tree,
			filesystem_trees: filesystem_trees,
			root_tree: root_tree,
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

		// read filesystem trees

		let filesystem_trees = {

			let output_job =
				output_job_start! (
					output,
					"Reading filesystem trees");

			let filesystem_trees =
				Self::read_filesystem_trees (
					output,
					& devices,
					& chunk_tree,
					& root_tree,
				) ?;

			output_job.complete ();

			filesystem_trees

		};

		// return

		Ok (BtrfsFilesystem {
			devices: devices,
			chunk_tree: chunk_tree,
			filesystem_trees: filesystem_trees,
			root_tree: root_tree,
		})

	}

	pub fn read_filesystem_trees <'b> (
		output: & Output,
		devices: & 'a BtrfsDeviceSet,
		chunk_tree: & 'b BtrfsChunkTree <'b>,
		root_tree: & 'b BtrfsRootTree <'b>,
	) -> Result <HashMap <u64, BtrfsFilesystemTree <'a>>, String> {

		let mut filesystem_trees: HashMap <u64, BtrfsFilesystemTree <'a>> =
			HashMap::new ();

		for root_item in root_tree.subvolume_root_items () {

			filesystem_trees.insert (
				root_item.object_id (),
				Self::read_filesystem_tree (
					output,
					devices,
					chunk_tree,
					& root_item,
				) ?,
			);

		}

		Ok (filesystem_trees)

	}

	pub fn read_filesystem_tree <'b> (
		output: & Output,
		devices: & 'a BtrfsDeviceSet,
		chunk_tree: & 'b BtrfsChunkTree <'b>,
		root_item: & 'b BtrfsRootItem <'b>,
	) -> Result <BtrfsFilesystemTree <'a>, String> {

		let output_job =
			output_job_start! (
				output,
				"Reading filesystem tree {}",
				root_item.object_id ());

		let filesystem_tree =
			BtrfsFilesystemTree::read_logical_address (
				output,
				devices,
				chunk_tree,
				root_item.root_node_block_number (),
			) ?;

		output_job.complete ();

		Ok (filesystem_tree)

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

	pub fn filesystem_tree (
		& 'a self,
		tree_id: u64,
	) -> Option <& 'a BtrfsFilesystemTree <'a>> {

		self.filesystem_trees.get (
			& tree_id,
		)

	}

	pub fn default_root_dir_item_entry (
		& 'a self,
	) -> Option <BtrfsDirItemEntry <'a>> {
		self.root_tree.default_root_dir_item_entry ()
	}

	pub fn default_root_inode_item (
		& 'a self,
	) -> Option <BtrfsInodeItem <'a>> {
		self.root_tree.default_root_inode_item ()
	}

	pub fn default_subvolume_root_item (
		& 'a self,
	) -> Option <BtrfsRootItem <'a>> {
		self.root_tree.default_subvolume_root_item ()
	}

	pub fn subvolume_root_refs (
		& 'a self,
	) -> Vec <BtrfsRootRef <'a>> {
		self.root_tree.subvolume_root_refs ()
	}

	pub fn subvolume_root_backrefs (
		& 'a self,
	) -> Vec <BtrfsRootBackref <'a>> {
		self.root_tree.subvolume_root_backrefs ()
	}

	pub fn root_item (
		& 'a self,
		tree_id: u64,
	) -> Option <BtrfsRootItem <'a>> {

		self.root_tree.root_item (
			tree_id,
		)

	}

	pub fn logical_to_physical_address (
		& self,
		logical_address: u64,
	) -> Option <BtrfsPhysicalAddress> {

		self.chunk_tree.logical_to_physical_address (
			logical_address,
		)

	}

	pub fn slice_at_physical_address (
		& self,
		physical_address: BtrfsPhysicalAddress,
		size: usize,
	) -> Result <& [u8], String> {

		let device =
			self.devices.device (
				physical_address.device_id (),
			).ok_or (

				format! (
					"Device not found: {}",
					physical_address.device_id ())

			) ?;

		device.slice_at (
			physical_address.offset () as usize,
			size,
		).ok_or (

			format! (
				"Physical address invalid for device {}: 0x{:x}",
				physical_address.device_id (),
				physical_address.offset ())

		)

	}

	pub fn slice_at_logical_address (
		& self,
		logical_address: u64,
		size: usize,
	) -> Result <& [u8], String> {

		let physical_address =
			self.logical_to_physical_address (
				logical_address,
			).ok_or (

				format! (
					"Logical address not valid: 0x{:x}",
					logical_address)

			) ?;

		self.slice_at_physical_address (
			physical_address,
			size,
		)

	}

	pub fn subvolume_path (
		& 'a self,
		root_backref: & 'a BtrfsRootBackref <'a>,
	) -> Result <PathBuf, String> {

		// TODO handle subvolume recursion

		let parent_filesystem_tree =
			self.filesystem_tree (
				root_backref.offset (),
			).ok_or (

				format! (
					"Can't find parent filesystem for subvolume {}",
					root_backref.object_id ())

			) ?;

		let mut path_parts: LinkedList <PathBuf> =
			LinkedList::new ();

		path_parts.push_front (
			PathBuf::from (
				OsStr::from_bytes (
					root_backref.name ())));

		let mut object_id =
			root_backref.directory_id ();

		loop {

			let parent_inode_ref =
				parent_filesystem_tree.inode_refs (
					object_id,
				).into_iter ().next ().ok_or (

					format! (
						"Can't find inode ref {}",
						object_id)

				) ?.entries ().next ().unwrap ();

			if parent_inode_ref.offset () == object_id {
				break;
			}

			path_parts.push_front (
				PathBuf::from (
					OsStr::from_bytes (
						parent_inode_ref.name ())));

			object_id =
				parent_inode_ref.offset ();

		}

		let mut path =
			PathBuf::from ("/");

		for path_part in path_parts {

			path.push (
				path_part);

		}

		Ok (path)

	}

}

// ex: noet ts=4 filetype=rust
