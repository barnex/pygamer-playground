//! Place a bitmap image on the screen. Convert png to .bmp
//! * Resize and export images directly from image editor by saving as .bmp and
//!   choosing 16bit R5 G6 B5
//! * OR Convert with imagemagick: convert rustacean-flat-noshadow.png -type
//!   truecolor -define bmp:subtype=RGB565 -depth 16 -strip -resize 86x64
//!   ferris.bmp

#![no_std]
#![no_main]

mod lib;
use lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

use hal::clock::GenericClockController;
use hal::gpio;
use hal::hal::spi;
use hal::sercom;
use hal::sercom::SPIMaster4;
use pac::SERCOM4;
use pac::{CorePeripherals, Peripherals};
use pygamer::{entry, hal, pac, Pins};

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Text;
use pygamer::gpio::Port;
use pygamer::prelude::*;
use pygamer::pwm;
use pygamer::sercom::PadPin;

use core::fmt::Write;
use st7735_lcd as lcd;
use tinybmp::Bmp;

fn init_display(
    display: pygamer::pins::Display,
    clocks: &mut GenericClockController,
    sercom4: pac::SERCOM4,
    mclk: &mut pac::MCLK,
    timer2: pac::TC2,
    delay: &mut hal::delay::Delay,
    port: &mut Port,
) -> Result<Display, ()> {
    let gclk0 = clocks.gclk0();
    let tft_spi = pygamer::sercom::SPIMaster4::new(
        &clocks.sercom4_core(&gclk0).ok_or(())?,
        16_u32.mhz(),
        spi::Mode {
            phase: spi::Phase::CaptureOnFirstTransition,
            polarity: spi::Polarity::IdleLow,
        },
        sercom4,
        mclk,
        (
            display.accel_irq.into_pad(port),
            display.tft_mosi.into_pad(port),
            display.tft_sck.into_pad(port),
        ),
    );

    let mut tft_cs = display.tft_cs.into_push_pull_output(port);
    tft_cs.set_low()?;

    let tft_dc = display.tft_dc.into_push_pull_output(port);
    let tft_reset = display.tft_reset.into_push_pull_output(port);


    let tft_backlight = display.tft_backlight.into_function_e(port);
    let mut pwm2 = pwm::Pwm2::new(
        &clocks.tc2_tc3(&gclk0).ok_or(())?,
        1.khz(),
        timer2,
        hal::pwm::TC2Pinout::Pa1(tft_backlight),
        mclk,
    );
    pwm2.set_duty(pwm2.get_max_duty());

    let mut display = st7735_lcd::ST7735::new(tft_spi, tft_dc, tft_reset, true, false, 160, 128);
    display.init(delay)?;
    display.set_orientation(&lcd::Orientation::LandscapeSwapped)?;

    Ok(display)
}

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

    //let (mut display, _backlight) = pins
    //    .display
    //    .init(
    //        &mut clocks,
    //        peripherals.SERCOM4,
    //        &mut peripherals.MCLK,
    //        peripherals.TC2,
    //        &mut delay,
    //        &mut pins.port,
    //    )
    //    .unwrap();

    let mut display = init_display(
        pins.display,
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

fn main_loop(display: &mut Display) -> Result<(), MyErr> {
    let mut fb = FrameBuffer::new();
    let mut fb2 = FrameBuffer::new();
    fb2.clear(Rgb565::BLACK).unwrap();

    let mut console = heapless::String::<32>::new();

    //use std::io::Write;

    let mut frame = 0;
    loop {
        console.clear();

        writeln!(&mut console, "{frame}").unwrap();
        let text = Text::new(&console, Point::new(0, 10), text_stile());

        fb.clear(Rgb565::BLUE).unwrap();
        text.draw(&mut fb).unwrap();
        //display.draw_iter(fb.iter_pixels()).unwrap();
        upload(&fb, display)?;

        //text.draw(&mut fb).unwrap();
        //display.draw_iter(fb.iter_pixels()).unwrap();
        upload(&fb2, display)?;
        upload(&fb, display)?;
        upload(&fb2, display)?;

        frame += 4;
    }

    //let raw_image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("../ferris.bmp")).unwrap();
    //let ferris = Image::new(&raw_image, Point::new(32, 32));
    //ferris.draw(&mut display).unwrap();

    Ok(())
}

fn upload(src: &FrameBuffer, dst: &mut Display) -> Result<(), MyErr> {
    //dst.draw_iter(src.iter_pixels()).unwrap();
    let N = SCREEN_H * SCREEN_W / 2;
    dst.set_address_window(0, 0, SCREEN_W as u16, SCREEN_H as u16 / 2)
        .map_err(my_err)?;
    dst.write_pixels(src.inner.iter().flatten().take(N).map(|c| c.into_storage()))
        .map_err(my_err)?;
    Ok(())
}

fn my_err<E>(_e: E) -> MyErr {
    MyErr {}
}

fn text_stile() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&mono_font::ascii::FONT_6X10, Rgb565::WHITE)
}

fn clear(display: &mut Display) {
    unwrap(display.clear(Rgb565::BLACK))
}

fn unwrap<E>(r: Result<(), E>) {
    r.map_err(|_| MyErr {}).unwrap()
}

fn print(display: &mut Display, text: &str) {
    Text::new(text, Point::new(0, 10), text_stile())
        .draw(display)
        .map_err(|_| MyErr {})
        .unwrap();
}
