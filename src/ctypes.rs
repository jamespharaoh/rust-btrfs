use std::ffi;
use std::fmt;
use std::os::unix::ffi::OsStringExt;

pub const UUID_SIZE: usize = 16;
pub const DEVICE_PATH_NAME_MAX: usize = 1024;

pub const AVAIL_ALLOC_BIT_SINGLE: u64 = 1 << 48;

pub const BLOCK_GROUP_DATA: u64 = 1 << 0;
pub const BLOCK_GROUP_SYSTEM: u64 = 1 << 1;
pub const BLOCK_GROUP_METADATA: u64 = 1 << 2;

pub const BLOCK_GROUP_RAID0: u64 = 1 << 3;
pub const BLOCK_GROUP_RAID1: u64 = 1 << 4;
pub const BLOCK_GROUP_DUP: u64 = 1 << 5;
pub const BLOCK_GROUP_RAID10: u64 = 1 << 6;
pub const BLOCK_GROUP_RAID5: u64 = 1 << 7;
pub const BLOCK_GROUP_RAID6: u64 = 1 << 8;

pub const BLOCK_GROUP_RESERVED: u64 = AVAIL_ALLOC_BIT_SINGLE;

pub const BLOCK_GROUP_DATA_AND_METADATA: u64 = (
	BLOCK_GROUP_DATA
	| BLOCK_GROUP_METADATA
);

pub const BLOCK_GROUP_TYPE_MASK: u64 = (
	BLOCK_GROUP_DATA
	| BLOCK_GROUP_SYSTEM
	| BLOCK_GROUP_METADATA
);

pub const BLOCK_GROUP_TYPE_AND_RESERVED_MASK: u64 = (
	BLOCK_GROUP_TYPE_MASK
	| BLOCK_GROUP_RESERVED
);

pub const BLOCK_GROUP_PROFILE_MASK: u64 = (
	BLOCK_GROUP_RAID0
	| BLOCK_GROUP_RAID1
	| BLOCK_GROUP_RAID5
	| BLOCK_GROUP_RAID6
	| BLOCK_GROUP_DUP
	| BLOCK_GROUP_RAID10
);

#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlSpaceArgs {
	pub space_slots: u64,
	pub total_spaces: u64,
}

#[ repr (C) ]
#[ derive (Copy, Clone, Debug) ]
pub struct IoctlSpaceInfo {
	pub flags: u64,
	pub total_bytes: u64,
	pub used_bytes: u64,
}

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

#[ repr (C) ]
#[ derive (Copy) ]
pub struct IoctlDevicePath {
	pub path: [u8; DEVICE_PATH_NAME_MAX],
}

impl IoctlDevicePath {

	pub fn as_vec (
		& self,
	) -> Vec <u8> {

		self.path.iter ().cloned ().take_while (
			|item|
			* item != 0
		).collect ()

	}

	pub fn as_os_string (
		& self,
	) -> ffi::OsString {

		ffi::OsString::from_vec (
			self.as_vec ())

	}

}

impl Clone for IoctlDevicePath {

	fn clone (
		& self,
	) -> Self {

		IoctlDevicePath {
			path: self.path,
		}

	}

	fn clone_from (
		& mut self,
		source: & Self,
	) {

		self.path.clone_from_slice (
			& source.path)

	}

}

impl fmt::Debug for IoctlDevicePath {

	fn fmt (
		& self,
		formatter: & mut fmt::Formatter,
	) -> fmt::Result {

		let string_bytes =
			self.as_vec ();

		let rust_string =
			String::from_utf8_lossy (
				& string_bytes);

		rust_string.fmt (
			formatter)

	}

}

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
