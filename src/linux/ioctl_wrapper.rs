use linux::imports::*;

// ---------- btrfs

pub const BTRFS_IOCTL_MAGIC: u64 = 0x94;

ioctl! (
	write ioctl_defrag_range with BTRFS_IOCTL_MAGIC, 16;
	IoctlDefragRangeArgs);

ioctl! (
	readwrite ioctl_dev_info with BTRFS_IOCTL_MAGIC, 30;
	IoctlDevInfoArgs);

ioctl! (
	readwrite ioctl_file_dedupe_range with BTRFS_IOCTL_MAGIC, 54;
	IoctlFileDedupeRange);

ioctl! (
	read ioctl_fs_info with BTRFS_IOCTL_MAGIC, 31;
	IoctlFsInfoArgs);

ioctl! (
	readwrite ioctl_space_info with BTRFS_IOCTL_MAGIC, 20;
	IoctlSpaceArgs);

// ---------- other

ioctl! (
	readwrite ioctl_fiemap with 'f' as u64, 11;
	IoctlFiemap);

// ex: noet ts=4 filetype=rust
