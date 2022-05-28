#![no_std]
#![no_main]

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

pub use hal::clock::GenericClockController;
pub use hal::gpio;
pub use hal::sercom;
pub use hal::sercom::SPIMaster4;
pub use pac::SERCOM4;
pub use pac::{CorePeripherals, Peripherals};
pub use pygamer::{entry, hal, pac, Pins};

pub use embedded_graphics::draw_target::DrawTarget;
pub use embedded_graphics::image::Image;
pub use embedded_graphics::mono_font;
pub use embedded_graphics::mono_font::MonoTextStyle;
pub use embedded_graphics::pixelcolor::Rgb565;
pub use embedded_graphics::prelude::*;
pub use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
pub use embedded_graphics::text::Text;

pub use st7735_lcd as lcd;
pub use tinybmp::Bmp;


pub use super::display::*;