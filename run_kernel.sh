#!/bin/bash
qemu-system-x86_64 -serial stdio -m 512M -drive format=raw,file=target/x86_64-thek/debug/bootimage-kstd.bin
