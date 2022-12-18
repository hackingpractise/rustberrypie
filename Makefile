all: kernel7.img
kernel7.img: target/armv7a-none-eabi/release/kernel
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/release/kernel ./kernel7.img
target/armv7a-none-eabi/release/kernel:
	cargo build --release
clean: 
	cargo clean
	rm -f kernel7.img
