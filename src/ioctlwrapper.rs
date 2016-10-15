use ctypes;

pub const MAGIC: u64 = 0x94;

ioctl! (
	readwrite space_info
	with MAGIC, 20; ctypes::IoctlSpaceArgs);

ioctl! (
	read fs_info
	with MAGIC, 31; ctypes::IoctlFsInfoArgs);

ioctl! (
	readwrite dev_info
	with MAGIC, 30; ctypes::IoctlDevInfoArgs);

// ex: noet ts=4 filetype=rust
