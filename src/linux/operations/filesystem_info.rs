//! This module contains an interface to the kernel's file system info
//! functionality.
//!
//! This includes information about the various backing devices and their usage.
//! This information is essential to detect when a BTRFS filesystem is reaching
//! capacity, and a high level of usage may indicate that a balance operation
//! is required.

use linux::imports::*;
use nix::Errno as NixErrno;
use nix::Error as NixError;

// ---------- get filesystem info

pub fn get_filesystem_info (
	file_descriptor: libc::c_int,
) -> Result <FilesystemInfo, String> {

	// get filesystem info

	let mut c_fs_info_args =
		IoctlFsInfoArgs::new ();

	unsafe {

		ioctl_fs_info (
			file_descriptor,
			& mut c_fs_info_args as * mut IoctlFsInfoArgs)

	}.map_err (
		|error|

		format! (
			"Error getting btrfs filesystem information: {}",
			error)

	) ?;

	// return

	Ok (
		FilesystemInfo {

			max_id:
				c_fs_info_args.max_id,

			num_devices:
				c_fs_info_args.num_devices,

			filesystem_id:
				Uuid::from_bytes (
					& c_fs_info_args.filesystem_id,
				).unwrap (),

		}
	)

}

pub fn get_device_info (
	file_descriptor: libc::c_int,
	device_id: u64,
) -> Result <Option <DeviceInfo>, String> {

	let mut c_dev_info_args =
		IoctlDevInfoArgs::new ();

	c_dev_info_args.devid =
		device_id;

	match unsafe {

		ioctl_dev_info (
			file_descriptor,
			& mut c_dev_info_args as * mut IoctlDevInfoArgs)

	} {

		Err (NixError::Sys (NixErrno::ENODEV)) =>
			return Ok (None),

		Err (NixError::Sys (errno)) =>
			return Err (
				format! (
					"Os error {} getting device info",
					errno)),

		Err (error) =>
			return Err (
				format! (
					"Unknown error getting device info: {}",
					error)),

		_ => (),

	};

	Ok (Some (
		DeviceInfo {

		device_id:
			c_dev_info_args.devid,

		uuid: Uuid::from_bytes (
			& c_dev_info_args.uuid,
		).unwrap (),

		bytes_used:
			c_dev_info_args.bytes_used,

		total_bytes:
			c_dev_info_args.total_bytes,

		path:
			c_dev_info_args.path.as_os_string (),

	}))

}

pub fn get_device_infos (
	file_descriptor: libc::c_int,
	filesystem_info: & FilesystemInfo,
) -> Result <Vec <DeviceInfo>, String> {

	let mut device_infos: Vec <DeviceInfo> =
		vec! [];

	for device_id in 0 .. filesystem_info.max_id + 1 {

		let device_info_option =
			try! (
				get_device_info (
					file_descriptor,
					device_id));

		if device_info_option.is_none () {
			continue;
		}

		let device_info =
			device_info_option.unwrap ();

		device_infos.push (
			device_info);

	}

	Ok (device_infos)

}

// ex: noet ts=4 filetype=rust
