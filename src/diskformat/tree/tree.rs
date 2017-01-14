use std::collections::HashMap;

use diskformat::*;

pub struct BtrfsTree {
}

impl BtrfsTree {

	/*
	pub fn read_tree <'a> (
		devices: & HashMap <u64, & 'a [u8]>,
		superblock: & BtrfsSuperblock,
		chunk_tree: & BtrfsChunkTree,
		tree_logical_address: u64,
	) -> Result <HashMap <BtrfsKey, BtrfsLeafItem <'a>>, String> {

		let mut tree_items: HashMap <BtrfsKey, BtrfsLeafItem> =
			HashMap::new ();

		Self::read_tree_recurse (
			devices,
			superblock,
			chunk_tree,
			tree_logical_address,
			& mut tree_items,
		) ?;

		Ok (tree_items)

	}
	*/

	pub fn read_tree_recurse <'a> (
		devices: & 'a BtrfsDeviceMap,
		superblock: & BtrfsSuperblock,
		chunk_tree: & BtrfsChunkTree,
		logical_address: u64,
		tree_items: & mut HashMap <BtrfsKey, BtrfsLeafItem <'a>>,
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

		let (device_id, device_address) =
			logical_to_physical_result.unwrap ();

		let device =
			devices.get (
				& device_id,
			).expect ("Lookup device");

		let node_bytes =
			device.slice_at (
				device_address as usize,
				superblock.node_size () as usize,
			);

		let node_result =
			BtrfsNode::from_bytes (
				node_bytes,
			);

		if let Err (error) = node_result {

			format! (
				"Error reading node as 0x{:x}: {}",
				logical_address,
				error);

			return;

		}

		let node =
			node_result.unwrap ();

		match node {

			BtrfsNode::Internal (internal_node) => {

				for item in internal_node.items () {

					Self::read_tree_recurse (
						devices,
						superblock,
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
