//! Place a bitmap image on the screen. Convert png to .bmp
//! * Resize and export images directly from image editor by saving as .bmp and
//!   choosing 16bit R5 G6 B5
//! * OR Convert with imagemagick: convert rustacean-flat-noshadow.png -type
//!   truecolor -define bmp:subtype=RGB565 -depth 16 -strip -resize 86x64
//!   ferris.bmp

#![no_std]
#![no_main]

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer::{entry, hal, pac, Pins};

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Text;
use hal::clock::GenericClockController;
use pac::{CorePeripherals, Peripherals};
use tinybmp::Bmp;

const SCREEN_W: i32 = 160;
const SCREEN_H: i32 = 128;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = Pins::new(peripherals.PORT).split();
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();

    match main_loop(&mut display) {
        Ok(()) => {
            clear(&mut display);
            print(&mut display, "done");
        }
        Err(_) => {
            clear(&mut display);
            print(&mut display, "error");
        }
    }

    loop {}
}

#[derive(Debug)]
struct MyErr {}

fn main_loop(display: &mut impl DrawTarget<Color = Rgb565>) -> Result<(), MyErr> {
    loop {
        clear(display);
        Text::new(
            "Hello Rust!\nNewline\nA veeeeeery long line",
            Point::new(0, 10),
            text_stile(),
        )
        .draw(display)
        .map_err(|_| MyErr {})?;
    }

    //let raw_image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("../ferris.bmp")).unwrap();
    //let ferris = Image::new(&raw_image, Point::new(32, 32));
    //ferris.draw(&mut display).unwrap();

    Ok(())
}

fn text_stile() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&mono_font::ascii::FONT_6X10, Rgb565::WHITE)
}

fn clear(display: &mut impl DrawTarget<Color = Rgb565>) {
    unwrap(display.clear(Rgb565::BLACK))
    //let bg = Rectangle::with_corners(Point::new(0, 0), Point::new(SCREEN_W, SCREEN_H)).into_styled(
    //    PrimitiveStyleBuilder::new()
    //        .fill_color(Rgb565::BLACK)
    //        .build(),
    //);
    //bg.draw(display).map_err(|_| MyErr {})
}

fn unwrap<E>(r: Result<(), E>) {
    r.map_err(|_| MyErr {}).unwrap()
}

fn print(display: &mut impl DrawTarget<Color = Rgb565>, text: &str) {
    Text::new(text, Point::new(0, 10), text_stile())
        .draw(display)
        .map_err(|_| MyErr {})
        .unwrap();
}
