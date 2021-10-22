# TheK

Unikernel specifically designed to serve as a backend for Rust std.

# Build

First run the following commands inside your project's folder:

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

Rust nightly compiler can be unstable and crash sometimes. In rare cases you will need to regenerate the project:

```
$ cargo clean
```