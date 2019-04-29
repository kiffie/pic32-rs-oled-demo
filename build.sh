#!/bin/bash

#make -C native_lib
RUST_TARGET_PATH=$(pwd) xargo build --target mipsel-none --release

pic32-bin2hex target/mipsel-none/release/oled-test
