# pic32-rs-oled-demo
Shows a moving Rust logo on an OLED display

Before the logo is shown text strings in different font sizes are displayed for
a few seconds.

This example also demonstrates some logging via the UART (port RPA0 at pin2 on
28 pin devices, 115200 bits/s).

Building requires `xargo` and `cargo-binutils`, which can be installed with
`cargo install`. A build can be done with

    RUST_TARGET_PATH=$(pwd) xargo objcopy --bin oled-test --release -- -O ihex oled-test.hex

See also the script `build.sh`

![Pic 32 OLED](https://raw.githubusercontent.com/kiffie/pic32-rs-oled-demo/master/doc/pic32-oled.jpg)
