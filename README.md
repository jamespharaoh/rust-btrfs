# Rust BTRFS library

Hosted at [gitlab.wellbehavedsoftware.com]
(https://gitlab.wellbehavedsoftware.com/well-behaved-software/rust-btrfs)

Mirrord at [github.com](https://github.com/wellbehavedsoftware/rust-btrfs)

Available from [crates.io](https://crates.io/crates/btrfs)

Written by [James Pharaoh](mailto:james@wellbehavedsoftware.com)

Open sourced under the [Apache 2.0 license]
(http://www.apache.org/licenses/LICENSE-2.0)

## Description

This is a (somewhat incomplete) rust reimplementation of the rust userspace
library. In fact, there isn't a C userspace library as such, but the userspace
tools include lowlevel interfaces which this project is based on.

This is mostly here to implement the things I need for now, but I'm open to any
contributions to make this the standard BTRFS userspace library for rust!

## Other links

[BTRFS wiki](https://btrfs.wiki.kernel.org/index.php/Main_Page)

[BTRFS utilities (kdave)]
(git://git.kernel.org/pub/scm/linux/kernel/git/kdave/btrfs-progs.git)

[BTRFS utilities (mason)]
(git://git.kernel.org/pub/scm/linux/kernel/git/mason/btrfs-progs.git)
