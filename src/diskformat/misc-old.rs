use std::collections::HashMap;
use std::slice;

pub const BTRFS_CHILD_UNKNOWN_TYPE: u8 = 0;
pub const BTRFS_CHILD_REGULAR_FILE_TYPE: u8 = 1;
pub const BTRFS_CHILD_DIRECTORY_TYPE: u8 = 2;
pub const BTRFS_CHILD_CHARACTER_DEVICE_TYPE: u8 = 3;
pub const BTRFS_CHILD_BLOCK_DEVICE_TYPE: u8 = 4;
pub const BTRFS_CHILD_FIFO_TYPE: u8 = 5;
pub const BTRFS_CHILD_SOCKET_TYPE: u8 = 6;
pub const BTRFS_CHILD_SYMBOLIC_LINK_TYPE: u8 = 7;
pub const BTRFS_CHILD_EXTENDED_ATTRIBUTE_TYPE: u8 = 8;

// ex: noet ts=4 filetype=rust
