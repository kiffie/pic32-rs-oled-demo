#!/bin/bash

# Use xargo and cargo-binutils to generate an IHEX file to be flashed
RUST_TARGET_PATH=$(pwd) \
	xargo objcopy --bin oled-test --release -- -O ihex oled-test.hex


# Use xargo to build an ELF file and the ChipKIT (or XC32) toolchain to convert
# the ELF file to an IHEX file
#RUST_TARGET_PATH=$(pwd) xargo build --target mipsel-none --release
#pic32-bin2hex target/mipsel-none/release/oled-test
