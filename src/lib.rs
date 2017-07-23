//! This library contains a partial implementation of a library for operating
//! with BTRFS filesystems. It is based on the C implementations of the BTRFS
//! utilities.
//!
//! It's home page is at [gitlab.wellbehavedsoftware.com]
//! (https://gitlab.wellbehavedsoftware.com/well-behaved-software/rust-btrfs).

#![ allow (unused_parens) ]

#[ macro_use ]
extern crate lazy_static;

#[ macro_use ]
extern crate nix;

#[ macro_use ]
extern crate output;

extern crate chrono;
extern crate crc;
extern crate itertools;
extern crate libc;
extern crate flate2;
extern crate memmap;
extern crate minilzo;
extern crate uuid;

pub mod compress;
pub mod diskformat;
pub mod linux;

pub use linux::*;

// ex: noet ts=4 filetype=rust
