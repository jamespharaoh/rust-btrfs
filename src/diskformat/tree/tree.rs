use std::collections::BTreeMap;

use diskformat::*;

pub struct BtrfsTree <'a> {
	tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
}

impl <'a> BtrfsTree <'a> {

	pub fn read_tree_logical_address (
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		logical_address: u64,
	) -> Result <BtrfsTree <'a>, String> {

		let (btrfs_tree, errors) =
			Self::read_tree_tolerant_logical_address (
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

	pub fn read_tree_physical_address (
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		physical_address: BtrfsPhysicalAddress,
	) -> Result <BtrfsTree <'a>, String> {

		let (btrfs_tree, errors) =
			Self::read_tree_tolerant_physical_address (
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

	pub fn read_tree_tolerant_physical_address (
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		physical_address: BtrfsPhysicalAddress,
	) -> (BtrfsTree <'a>, Vec <String>) {

		let mut tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem> =
			BTreeMap::new ();

		let mut errors: Vec <String> =
			Vec::new ();

		Self::read_tree_recurse_physical_address (
			devices,
			chunk_tree,
			physical_address,
			& mut tree_items,
			& mut errors,
		);

		(
			BtrfsTree {
				tree_items: tree_items,
			},
			errors,
		)

	}

	pub fn read_tree_tolerant_logical_address (
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		logical_address: u64,
	) -> (BtrfsTree <'a>, Vec <String>) {

		let mut tree_items: BTreeMap <BtrfsKey, BtrfsLeafItem> =
			BTreeMap::new ();

		let mut errors: Vec <String> =
			Vec::new ();

		Self::read_tree_recurse_logical_address (
			devices,
			chunk_tree,
			logical_address,
			& mut tree_items,
			& mut errors,
		);

		(
			BtrfsTree {
				tree_items: tree_items,
			},
			errors,
		)

	}

	pub fn read_tree_recurse_logical_address (
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		logical_address: u64,
		tree_items: & mut BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
		errors: & mut Vec <String>,
	) {

		let logical_to_physical_result =
			chunk_tree.logical_to_physical_address (
				logical_address,
			);

		if logical_to_physical_result.is_none () {

			errors.push (
				format! (
					"Logical address lookup failed: 0x{:x}",
					logical_address));

			return;

		}

		let physical_address =
			logical_to_physical_result.unwrap ();

		Self::read_tree_recurse_physical_address (
			devices,
			chunk_tree,
			physical_address,
			tree_items,
			errors,
		)

	}

	pub fn read_tree_recurse_physical_address (
		devices: & 'a BtrfsDeviceSet <'a>,
		chunk_tree: & BtrfsChunkTree,
		physical_address: BtrfsPhysicalAddress,
		tree_items: & mut BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
		errors: & mut Vec <String>,
	) {

		let device =
			devices.device (
				physical_address.device_id (),
			).expect ("Lookup device");

		let node_bytes_result =
			device.slice_at (
				physical_address.offset () as usize,
				devices.superblock ().node_size () as usize,
			);

		if node_bytes_result.is_none () {

			errors.push (
				format! (
					"Physical address range invalid on device {}: 0x{:x} to \
					0x{:x}",
					physical_address.device_id (),
					physical_address.offset (),
					physical_address.offset ()
						+ devices.superblock ().node_size () as u64));

			return;

		}

		let node_bytes =
			node_bytes_result.unwrap ();

		let node_result =
			BtrfsNode::from_bytes (
				physical_address,
				node_bytes,
			);

		if let Err (error) = node_result {

			errors.push (
				format! (
					"Error reading node at {}: {}",
					physical_address,
					error));

			return;

		}

		let node =
			node_result.unwrap ();

		match node {

			BtrfsNode::Internal (internal_node) => {

				for item in internal_node.items () {

					Self::read_tree_recurse_logical_address (
						devices,
						chunk_tree,
						item.key ().offset (),
						tree_items,
						errors,
					);

				}

			},

			BtrfsNode::Leaf (leaf_node) => {

				for item in leaf_node.items () {

					tree_items.insert (
						item.key (),
						item,
					);

				}

			},

		}

	}

}

// ex: noet ts=4 filetype=rust
