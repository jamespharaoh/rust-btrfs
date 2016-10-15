use ctypes;

pub const MAGIC: u64 = 0x94;

ioctl! (
	readwrite dev_info
	with MAGIC, 30; ctypes::IoctlDevInfoArgs);

ioctl! (
	readwrite file_dedupe_range
	with MAGIC, 54; ctypes::IoctlFileDedupeRange);

ioctl! (
	read fs_info
	with MAGIC, 31; ctypes::IoctlFsInfoArgs);

ioctl! (
	readwrite space_info
	with MAGIC, 20; ctypes::IoctlSpaceArgs);

// ex: noet ts=4 filetype=rust
