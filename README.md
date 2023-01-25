# rust-minimal-kernel

To build the disk image
```zsh
cargo bootimage
```

To run the disk image on QEMU
```zsh
qemu-system-x86_64 -drive format=raw,file=target/rust_minimal_kernel/debug/bootimage-rust_minimal_kernel.bin
```
