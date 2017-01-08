//! This module contains the public types used by this library.
//!
//! Many of these closely match the C types used internally when interfacing
//! with the kernal, but they are desiggned to be idomatically Rust and far
//! simpler to use.

mod compression_type;
mod dedupe_range;
mod dedupe_range_dest_info;
mod dedupe_range_status;
mod device_info;
mod file_descriptor;
mod filesystem_info;
mod group_profile;
mod group_type;
mod space_info;

pub use self::compression_type::*;
pub use self::dedupe_range::*;
pub use self::dedupe_range_dest_info::*;
pub use self::dedupe_range_status::*;
pub use self::device_info::*;
pub use self::file_descriptor::*;
pub use self::filesystem_info::*;
pub use self::group_profile::*;
pub use self::group_type::*;
pub use self::space_info::*;

// ex: noet ts=4 filetype=rust
