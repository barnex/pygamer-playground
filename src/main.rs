#![no_std]
#![no_main]

mod lib;
use lib::display::*;
use lib::hw::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

use core::fmt::Write;

use pygamer as bsp;

use bsp::buttons::Keys;
use bsp::entry;
use bsp::prelude::*;

use lis3dh::accelerometer::vector::F32x3;
use lis3dh::accelerometer::Accelerometer;

use embedded_graphics as eg;

use eg::draw_target::DrawTarget;
use eg::image::Image;
use eg::mono_font;
use eg::mono_font::MonoTextStyle;
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::text::Text;

use tinybmp::Bmp;


#[entry]
fn main() -> ! {
    let mut hw = HW::new();

    lib::app::nomnom::main(&mut hw);

    loop {}
}
