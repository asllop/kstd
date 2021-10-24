#!/bin/bash
qemu-system-x86_64 -m 512M -drive format=raw,file=target/x86_64-thek/debug/bootimage-thek.bin
