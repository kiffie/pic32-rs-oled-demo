//! OLED test


#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::fmt::Write;
use core::cell::Cell;
use tinylog;
use tinylog::{debug, info, error};
use mips_rt;
pub use pic32mx1xxfxxxb as pac;

use mips_rt::interrupt;

use pic32_hal::uart;
use pic32_hal::uart::Uart;
use pic32_hal::i2c;
use pic32_hal::cp0timer;
use pic32_hal::cp0timer::time_from_secs;

use ssd1306::Builder;
use ssd1306::mode::GraphicsMode;

use embedded_graphics::image::Image1BPP;
use embedded_graphics::prelude::*;

const TL_LOGLEVEL: tinylog::Level = tinylog::Level::Debug;

// PIC32 configuration registers
#[link_section = ".configsfrs"]
#[no_mangle]
pub static CONFIGSFRS: [u32; 4] = [
    0xdfffffff,     // DEVCFG3
    0xfff9ffd9,     // DEVCFG2
    0xff7fcfd9,     // DEVCFG1
    0x7ffffffb,     // DEVCFG0
];


struct TxWriter<'a> {
    tx: &'a uart::Tx,
}

impl<'a> TxWriter<'a> {
    fn new(tx: &uart::Tx) -> TxWriter {
        TxWriter{
            tx: tx,
        }
    }
}

impl<'a> core::fmt::Write for TxWriter<'a> {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            while !self.tx.try_write_byte(b) {};
        }
        Ok(())
    }

}


struct UartLogger {
    tx: Option<uart::Tx>,
}

impl UartLogger {
    const fn new() -> UartLogger {
        UartLogger{
            tx: None,
        }
    }

    fn set_tx(&mut self, tx: uart::Tx) {
        self.tx = Some(tx);
    }
}

impl tinylog::Log for UartLogger {

    fn log(&self, args: core::fmt::Arguments) {
        if let Some(ref tx) = self.tx {
            let mut txw = TxWriter::new(tx);
            writeln!(txw, "{}", args).unwrap();
        }
    }

    fn flush(&self) {}
}

fn get_led() -> bool {
    let  p = unsafe { pic32mx1xxfxxxb::Peripherals::steal()};
    p.PORTB.latb.read().latb4().bit()
}

fn set_led(on: bool){
    let  p = unsafe { pic32mx1xxfxxxb::Peripherals::steal()};
    p.PORTB.trisbclr.write(|w| {w.trisb4().bit(true)});
    if on {
        p.PORTB.latbset.write(|w| {w.latb4().bit(true)});
    }else{
        p.PORTB.latbclr.write(|w| {w.latb4().bit(true)});
    }
}


static mut UART_LOGGER: UartLogger = UartLogger::new();

pub const SYS_CLOCK: u32 = 40000000;

fn timer_handler(when: cp0timer::Time){
    set_led(!get_led());
    let mut timer = cp0timer::Timer::new();
    timer.at(when + time_from_secs(1), timer_handler).unwrap();
}


#[no_mangle]
pub fn main() -> ! {

    //configure IO ports for UART2
    let  p = unsafe { pic32mx1xxfxxxb::Peripherals::steal()};
    let pps = p.PPS;
    pps.rpa0r.write(|w| unsafe { w.rpa0r().bits(0b0001) }); // U1TX on RPA0
    // initialize UART1
    let uart = Uart::new(uart::HwModule::UART1);
    uart.init(SYS_CLOCK, 115200);
    let (tx, _) = uart.split();

    unsafe {
        UART_LOGGER.set_tx(tx);
        tinylog::set_logger(&UART_LOGGER);
    }
    unsafe {
        interrupt::enable_mv_irq();
        interrupt::enable();
    }



    let mut state = false;

    set_led(true);

    info!("initializing display");
    let mut i2c = i2c::I2c::new(i2c::HwModule::I2C1);
    i2c.init(SYS_CLOCK, i2c::Fscl::F400KHZ);
    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();
    disp.flush().unwrap();


    let bitmap = include_bytes!("./rust.raw");

    info!("starting loop");
    let mut x = 0;
    let mut move_right = true;

    let mut timer = cp0timer::Timer::new();
    timer.at(timer.now() + time_from_secs(1), timer_handler).unwrap();

    for i in 1..10 {
        set_led( i & 0x01 != 0);
        timer.delay_millis(100);
    }

    loop {
        let im = Image1BPP::new(bitmap, 64, 64).translate(Coord::new(x, 0));
        disp.clear();
        disp.draw(im.into_iter());
        disp.flush().unwrap();
        state = !state;
        if move_right {
            if x < 64 {
                x += 1;
            }else{
                debug!("left, seconds = {}", timer.now_secs());
                move_right = false;
            }
        }else {
            if x > 0 {
                x -= 1;
            }else{
                debug!("right");
                move_right = true;
            }
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    error!("Panic: entering endless loop");
    loop {}
}
