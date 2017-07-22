use std::collections::HashMap;
use std::mem;

use super::super::*;

pub struct BtrfsDeviceSet <'a> {
	devices: HashMap <u64, BtrfsDevice <'a>>,
	superblock: BtrfsSuperblock <'a>,
}

impl <'a> BtrfsDeviceSet <'a> {

	pub fn new (
		datas: & [& 'a [u8]],
	) -> Result <BtrfsDeviceSet <'a>, String> {

		if datas.len () == 0 {

			return Err (
				format! (
					"No device data supplied"));

		}

		let mut devices =
			HashMap::new ();

		let mut superblocks: Vec <BtrfsSuperblock <'a>> =
			Vec::new ();

		for data in datas {

			let btrfs_device =
				BtrfsDevice::new (
					data,
				) ?;

			if devices.contains_key (
				& btrfs_device.device_id ()) {

				return Err (
					format! (
						"Multiple devices with id {}",
						btrfs_device.device_id ()));

			}

			superblocks.push (
				BtrfsSuperblock::for_bytes (
					& data [
						btrfs_device.superblock_offset ()
					..
						btrfs_device.superblock_offset ()
							+ mem::size_of::<BtrfsSuperblockData> ()
					],
				),
			);

			devices.insert (
				btrfs_device.device_id (),
				btrfs_device);

		}

		let superblock =
			superblocks.into_iter ().max_by_key (
				|superblock| superblock.generation ()
			).unwrap ();

		Ok (BtrfsDeviceSet {
			devices: devices,
			superblock: superblock,
		})

	}

	pub fn device (
		& 'a self,
		device_id: u64,
	) -> Option <& 'a BtrfsDevice <'a>> {

		self.devices.get (
			& device_id,
		)

	}

	pub fn system_logical_to_physical (
		& self,
		logical_address: u64,
	) -> Option <BtrfsPhysicalAddress> {

		self.superblock.system_logical_to_physical (
			logical_address,
		)

	}

	pub fn system_slice_at_logical_address (
		& self,
		logical_address: u64,
		size: usize,
	) -> Result <& [u8], String> {

		let physical_address =
			self.system_logical_to_physical (
				logical_address,
			).ok_or (

				format! (
					"Logical address not valid: 0x{:x}",
					logical_address)

			) ?;

		self.system_slice_at_physical_address (
			physical_address,
			size,
		)

	}

	pub fn system_slice_at_physical_address (
		& self,
		physical_address: BtrfsPhysicalAddress,
		size: usize,
	) -> Result <& [u8], String> {

		let device =
			self.devices.get (
				& physical_address.device_id (),
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

	pub fn node_at_physical_address (
		& self,
		physical_address: BtrfsPhysicalAddress,
	) -> Result <BtrfsNode, String> {

		let device =
			self.devices.get (
				& physical_address.device_id (),
			).ok_or (

				format! (
					"Device not found: {}",
					physical_address.device_id ())

			) ?;

		device.node_at (
			physical_address.offset (),
		)

	}

	pub fn superblock (& 'a self) -> BtrfsSuperblock <'a> {
		self.superblock
	}

}

// ex: noet ts=4 filetype=rust
