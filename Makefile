
RUSTFLAGS_PEDANTIC = -D warnings

all: kernel7.img
kernel7.img: target/armv7a-none-eabi/release/kernel
<<<<<<< Updated upstream
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/release/kernel ./kernel7.img
target/armv7a-none-eabi/release/kernel:
=======
	rust-objcopy -O binary target/armv7a-none-eabi/release/kernel ./kernel7.img
target/armv7a-none-eabi/release/kernel: clippy
>>>>>>> Stashed changes
	cargo build --release
clean: 
	cargo clean
	rm -f kernel7.img
<<<<<<< Updated upstream
=======
objdump: target/armv7a-none-eabi/release/kernel
	cargo objdump --bin kernel --release -- --disassemble --no-show-raw-insn --print-imm-hex

clippy:
	cargo clippy -- -D clippy::pedantic 
doc:
	cargo doc --open
>>>>>>> Stashed changes
