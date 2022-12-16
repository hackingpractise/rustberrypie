# RUSTBERRYPI
It is a crate that blinks an LED
in the raspberry pi 3. Run make
to compile the binary that will
flashed.

## Requirements
1. arm-none-eabi-objcopy arm
2. rust installed in the local system and the armv7a-none-eabi
   toolchain.
## SOURCES
1. https://www.youtube.com/watch?v=jZT8APrzvc4
2. https://www.youtube.com/watch?v=mPB3dCWlZVY
3. https://docs.rust-embedded.org/book/
## Build
1. Run cargo build --release to build the binary.
2. Run cargo objdump --bin rustberrypie -- --disassemble --no-show-raw-insn --print-imm-hex to see the disassembled bin.
 
