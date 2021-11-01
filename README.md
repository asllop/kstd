# K-std

Unikernel specifically designed to serve as a backend for Rust std. It's composed by two parts:

- The std library, that is essentially a stripped down version of the official rust std.
- The K, that is the actual kernel that provides the low level funcions requiered by std.

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