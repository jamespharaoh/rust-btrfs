use linux::ctypes::ioctl_constants::*;

#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlFsInfoArgs {
	pub max_id: u64,
	pub num_devices: u64,
	pub filesystem_id: [u8; UUID_SIZE],
	pub reserved0: [u64; 32],
	pub reserved1: [u64; 32],
	pub reserved2: [u64; 32],
	pub reserved3: [u64; 28],
}

impl IoctlFsInfoArgs {

	pub fn new (
	) -> IoctlFsInfoArgs {

		IoctlFsInfoArgs {
			max_id: 0,
			num_devices: 0,
			filesystem_id: [0u8; 16],
			reserved0: [0u64; 32],
			reserved1: [0u64; 32],
			reserved2: [0u64; 32],
			reserved3: [0u64; 28],
		}

	}

}

// ex: noet ts=4 filetype=rust
