//! This library contains a partial implementation of a library for operating
//! with BTRFS filesystems. It is based on the C implementations of the BTRFS
//! utilities.
//!
//! It's home page is at [gitlab.wellbehavedsoftware.com]
//! (https://gitlab.wellbehavedsoftware.com/well-behaved-software/rust-btrfs).

#[ macro_use ]
extern crate ioctl;

extern crate libc;
extern crate uuid;

pub mod deduplicate;
pub mod fiemap;
pub mod filesysteminfo;
pub mod spaceinfo;
pub mod types;

mod ctypes;
mod filedescriptor;
mod ioctlwrapper;

pub use deduplicate::*;
pub use fiemap::*;
pub use filesysteminfo::*;
pub use spaceinfo::*;
pub use types::*;

// ex: noet ts=4 filetype=rust
