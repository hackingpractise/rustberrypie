default:
	cargo clean
	cargo build --release
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/release/rustberrypie ./kernel7.img
clean: 
	cargo clean
	rm -f kernel7.img
