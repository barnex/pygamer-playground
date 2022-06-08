#![no_std]
#![no_main]

mod lib;
use bsp::entry;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use lib::app;
use lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut sys = Sys::new();

    let opts = [
        "Nom Nom Cat", //
        "Sensors",
    ];

    loop {
        let selection = sys.show_menu(&opts);

        match selection {
            0 => app::nomnom::main(&mut sys),
            1 => app::sensors::main(&mut sys.hw),
            _ => (),
        }
    }
}
