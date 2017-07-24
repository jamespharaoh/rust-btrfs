use std::collections::BTreeMap;

use output::*;

use super::super::*;

pub fn btrfs_read_tree_recurse_physical_address <'a> (
	output: & Output,
	devices: & 'a BtrfsDeviceSet <'a>,
	chunk_tree: & BtrfsChunkTree,
	physical_address: BtrfsPhysicalAddress,
	tree_items: & mut BTreeMap <BtrfsKey, BtrfsLeafItem <'a>>,
	errors: & mut Vec <String>,
) {

	output_message! (
		output,
		"Read tree node at {}",
		physical_address);

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

		let error =
			format! (
				"Physical address range invalid on device {}: 0x{:x} to \
				0x{:x}",
				physical_address.device_id (),
				physical_address.offset (),
				physical_address.offset ()
					+ devices.superblock ().node_size () as u64);

		output_message! (
			output,
			"{}",
			error);

		errors.push (
			error);

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

		let error =
			format! (
				"Error reading node at {}: {}",
				physical_address,
				error);

		output_message! (
			output,
			"{}",
			error);

		errors.push (
			error);

		return;

	}

	let node =
		node_result.unwrap ();

	match node {

		BtrfsNode::Internal (internal_node) => {

			for item in internal_node.items () {

				btrfs_read_tree_recurse_logical_address (
					output,
					devices,
					chunk_tree,
					item.block_number (),
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

pub fn btrfs_read_tree_recurse_logical_address <'a> (
	output: & Output,
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

		let error =
			format! (
				"Logical address lookup failed: 0x{:x}",
				logical_address);

		output_message! (
			output,
			"{}",
			error);

		errors.push (
			error);

		return;

	}

	let physical_address =
		logical_to_physical_result.unwrap ();

	btrfs_read_tree_recurse_physical_address (
		output,
		devices,
		chunk_tree,
		physical_address,
		tree_items,
		errors,
	)

}

// ex: noet ts=4 filetype=rust
