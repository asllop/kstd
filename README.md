# TheK

Unikernel specifically design to serve as a backend for Rust std.

# Build

First install rustc nightly and some components. Run this inside project's dir:

```
$ rustup override set nightly
$ rustup component add rust-src
$ rustup component add llvm-tools-preview
$ cargo install bootimage
```

And finally build the image:

```
$ cargo bootimage
```

This will generate a bootable image you can run with QEMU:

```
$ sh run_kernel.sh
```
