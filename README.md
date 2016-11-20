# Rust BTRFS library

Home page at [rust-btrfs.com](http://rust-btrfs.com)

Git repository at [gitlab.wellbehavedsoftware.com]
(https://gitlab.wellbehavedsoftware.com/well-behaved-software/rust-btrfs)

Mirrord at [github.com](https://github.com/wellbehavedsoftware/rust-btrfs)

Available from [crates.io](https://crates.io/crates/btrfs)

Written by [James Pharaoh](mailto:james@wellbehavedsoftware.com)

Documentation at [docs.rs](https://docs.rs/btrfs/)

Open sourced under the permissive [MIT licence]
(https://opensource.org/licenses/MIT)

## Description

This is a (somewhat incomplete) rust reimplementation of the rust userspace
library. In fact, there isn't a C userspace library as such, but the userspace
tools include lowlevel interfaces which this project is based on.

This is mostly here to implement the things I need for now, but I'm open to any
contributions to make this the standard BTRFS userspace library for rust!

## Supported features

This library consists of a number of wrappers around the BTRFS ioctls.

- Deduplication (not BTRFS specific)
- Fiemap (file extent map, not BTRFS specific)
- File system info
- Space and device info

## Other links

[BTRFS wiki](https://btrfs.wiki.kernel.org/index.php/Main_Page)

[BTRFS utilities (kdave)]
(git://git.kernel.org/pub/scm/linux/kernel/git/kdave/btrfs-progs.git)

[BTRFS utilities (mason)]
(git://git.kernel.org/pub/scm/linux/kernel/git/mason/btrfs-progs.git)
