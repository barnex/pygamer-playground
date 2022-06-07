#![no_std]
#![no_main]

mod lib;
use bsp::entry;
use lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut hw = HW::new();

    lib::app::nomnom::main(&mut hw);

    loop {}
}
