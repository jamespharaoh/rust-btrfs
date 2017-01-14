use std::collections::BTreeMap;
use std::collections::HashMap;

use diskformat::*;

pub type BtrfsChunkItemsByOffset <'a> =
	BTreeMap <u64, BtrfsChunkItem <'a>>;

pub struct BtrfsChunkTree <'a> {
	chunk_items_by_offset: BtrfsChunkItemsByOffset <'a>,
}

impl <'a> BtrfsChunkTree <'a> {

	pub fn new (
		devices: & 'a BtrfsDeviceMap,
		superblock: & BtrfsSuperblock,
	) -> Result <BtrfsChunkTree <'a>, String> {

		let extent_tree_items =
			Self::read_system_extent_tree (
				devices,
				superblock,
			) ?;

		let mut chunk_items_by_offset =
			BtrfsChunkItemsByOffset::new ();

		for extent_tree_item in extent_tree_items.values () {

			match extent_tree_item {

				& BtrfsLeafItem::ChunkItem (chunk_item) => {

					chunk_items_by_offset.insert (
						chunk_item.key ().offset (),
						chunk_item);

				},

				_ => (),

			}

		}

		Ok (
			BtrfsChunkTree {
				chunk_items_by_offset: chunk_items_by_offset,
			}
		)

	}

	fn read_system_extent_tree (
		devices: & 'a BtrfsDeviceMap,
		superblock: & BtrfsSuperblock,
	) -> Result <HashMap <BtrfsKey, BtrfsLeafItem <'a>>, String> {

		let mut extent_tree_items: HashMap <BtrfsKey, BtrfsLeafItem> =
			HashMap::new ();

		Self::read_system_extent_tree_recurse (
			devices,
			superblock,
			superblock.chunk_tree_logical_address (),
			& mut extent_tree_items,
		) ?;

		Ok (extent_tree_items)

	}

	fn read_system_extent_tree_recurse (
		devices: & 'a BtrfsDeviceMap,
		superblock: & BtrfsSuperblock,
		logical_address: u64,
		extent_tree_items: & mut HashMap <BtrfsKey, BtrfsLeafItem <'a>>,
	) -> Result <(), String> {

		let (device_id, device_address) =
			superblock.system_logical_to_physical (
				logical_address,
			).expect ("Lookup logical address");

		let device =
			devices.get (
				& device_id,
			).expect ("Lookup device");

		let node_bytes =
			device.slice_at (
				device_address as usize,
				superblock.node_size () as usize);

		let node =
			BtrfsNode::from_bytes (
				node_bytes,
			) ?;

		match node {

			BtrfsNode::Internal (internal_node) => {

				for item in internal_node.items () {

					Self::read_system_extent_tree_recurse (
						devices,
						superblock,
						item.key ().offset (),
						extent_tree_items,
					) ?;

				}

			},

			BtrfsNode::Leaf (leaf_node) => {

				for item in leaf_node.items () {

					extent_tree_items.insert (
						item.key (),
						item,
					);

				}

			},

		}

		Ok (())

	}

	pub fn logical_to_physical_address (
		& self,
		logical_address: u64,
	) -> Option <(u64, u64)> {

		// TODO waiting for range to land in stable rust

		for (ref chunk_item_offset, ref chunk_item)
		in self.chunk_items_by_offset.iter () {

			let chunk_item_offset =
				** chunk_item_offset;

			if logical_address >= chunk_item_offset
			&& logical_address < (
				chunk_item_offset +
				chunk_item.data ().chunk_size ()
			) {

				let chunk_item_stripe =
					chunk_item.stripes () [0];

				return Some (
					(
						chunk_item_stripe.device_id (),
						logical_address
							- chunk_item_offset
							+ chunk_item_stripe.offset (),
					)
				);

			}

		}

		None

	}

}

// ex: noet ts=4 filetype=rust
