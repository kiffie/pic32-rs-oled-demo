# pic32-rs-oled-demo
Shows a moving Rust logo on an OLED display

Building requires `xargo` and `cargo-binutils`, which can be installed with
`cargo install`. A build can be done with

    RUST_TARGET_PATH=$(pwd) xargo objcopy --bin oled-test --release -- -O ihex oled-test.hex

See also the script `build.sh`

![Pic 32 OLED](https://raw.githubusercontent.com/kiffie/pic32-rs-oled-demo/master/doc/pic32-oled.jpg)
