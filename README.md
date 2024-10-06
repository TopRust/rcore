# 脚本
cargo build --release
file target/riscv64gc-unknown-none-elf/release/os
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S

riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

# 问题
gdb莫名其妙地好了
在lang_items.rs中use crate::println;
在lang_items.rs中info.message().unwrap()替换为info.message().as_str().unwrap()
