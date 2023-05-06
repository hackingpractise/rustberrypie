all: kernel7.img
kernel7.img: target/armv7a-none-eabi/release/kernel
    rust-objcopy -O binary target/armv7a-none-eabi/release/kernel ./kernel7.img
target/armv7a-none-eabi/release/kernel:
    cargo build --release
clean: 
    cargo clean
    rm -f kernel7.img
objdump: target/armv7a-none-eabi/release/kernel
    cargo objdump --bin kernel --release -- --disassemble --no-show-raw-insn --print-imm-hex
