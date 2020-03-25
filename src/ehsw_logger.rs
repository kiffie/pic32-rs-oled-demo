//! Embedded Hal Serial::Write Logger (EHSWLogger)
//! A tiny looger implementaion based on the embedded_hal::serial::Write
//! trait

use core::cell::RefCell;
use core::fmt;
use core::fmt::Write as fmtWrite;

use mips_rt;

use mips_rt::interrupt;
use mips_rt::interrupt::Mutex;

use embedded_hal::serial::Write;

use log::{Metadata, Record};

struct SerialFmtWrite<'a, E> {
    w: &'a mut dyn Write<u8, Error = E>,
}

impl<E> SerialFmtWrite<'_, E> {
    fn new(w: &mut dyn Write<u8, Error = E>) -> SerialFmtWrite<E> {
        SerialFmtWrite { w }
    }
}

impl<E> fmtWrite for SerialFmtWrite<'_, E> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            while self.w.write(b).is_err() {}
        }
        Ok(())
    }
}

pub struct EhswLogger<TX: Send> {
    tx: Mutex<RefCell<Option<TX>>>,
}

/// An instance of a Logger using an object of the trait
/// embedded_hal::serial::Write<u8> for log output.
impl<TX> EhswLogger<TX>
where
    TX: Send,
{
    /// Create a new logger that that can be registered with the Log crate
    pub const fn new() -> EhswLogger<TX> {
        EhswLogger {
            tx: Mutex::new(RefCell::new(None)),
        }
    }

    pub fn set_tx(&self, tx: TX) {
        interrupt::free(|cs| {
            let cell = self.tx.borrow(cs);
            cell.replace(Some(tx));
        });
    }
}

impl<TX> log::Log for EhswLogger<TX>
where
    TX: Write<u8> + Send,
{
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        interrupt::free(|cs| {
            let cell = self.tx.borrow(cs);
            if let Some(ref mut tx) = *cell.borrow_mut() {
                let mut w = SerialFmtWrite::new(tx);
                writeln!(
                    w,
                    "[{:.1} {}:{}] {}",
                    record.level(),
                    record.file_static().unwrap_or("<NONE>"),
                    record.line().unwrap_or(0),
                    record.args()
                )
                .unwrap();
            }
        });
    }

    fn flush(&self) {}
}
