use linux::ctypes::ioctl_constants::*;
use linux::ctypes::ioctl_device_path::*;

#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlDevInfoArgs {
	pub devid: u64,
	pub uuid: [u8; UUID_SIZE],
	pub bytes_used: u64,
	pub total_bytes: u64,
	pub unused0: [u64; 32],
	pub unused1: [u64; 32],
	pub unused2: [u64; 32],
	pub unused3: [u64; 32],
	pub unused4: [u64; 32],
	pub unused5: [u64; 32],
	pub unused6: [u64; 32],
	pub unused7: [u64; 32],
	pub unused8: [u64; 32],
	pub unused9: [u64; 32],
	pub unused10: [u64; 32],
	pub unused11: [u64; 27],
	pub path: IoctlDevicePath,
}

impl IoctlDevInfoArgs {

	pub fn new (
	) -> IoctlDevInfoArgs {

		IoctlDevInfoArgs {
			devid: 0,
			uuid: [0; UUID_SIZE],
			bytes_used: 0,
			total_bytes: 0,
			unused0: [0; 32],
			unused1: [0; 32],
			unused2: [0; 32],
			unused3: [0; 32],
			unused4: [0; 32],
			unused5: [0; 32],
			unused6: [0; 32],
			unused7: [0; 32],
			unused8: [0; 32],
			unused9: [0; 32],
			unused10: [0; 32],
			unused11: [0; 27],
			path: IoctlDevicePath {
				path: [0; DEVICE_PATH_NAME_MAX],
			},
		}

	}

}

// ex: noet ts=4 filetype=rust
