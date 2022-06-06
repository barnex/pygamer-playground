#![no_std]
#![no_main]

mod lib;
use lib::display::*;
use lib::framebuffer::*;
//use lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

use core::fmt::Write;

use pac::gclk::pchctrl::GEN_A::GCLK11;
use pac::SERCOM4;
use pac::{CorePeripherals, Peripherals};

use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::gpio;
use hal::hal::spi;
use hal::sercom;
use hal::sercom::SPIMaster4;
use hal::time::KiloHertz;

use pygamer as bsp;

use bsp::buttons::Keys;
use bsp::gpio::Port;
use bsp::prelude::*;
use bsp::pwm;
use bsp::sercom::PadPin;
use bsp::{entry, hal, pac, Pins};

use lis3dh::accelerometer::vector::F32x3;
use lis3dh::{accelerometer::Accelerometer, Lis3dh};

use eg::prelude::*;
use embedded_graphics as eg;

use eg::draw_target::DrawTarget;
use eg::image::Image;
use eg::mono_font;
use eg::mono_font::MonoTextStyle;
use eg::pixelcolor::Rgb565;
use eg::primitives::PrimitiveStyleBuilder;
use eg::primitives::Rectangle;
use eg::text::Text;

use tinybmp::Bmp;

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
    let mut pins = Pins::new(peripherals.PORT);
    let mut pins = pins.split();

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

    //let mut display = init_display(
    //    pins.display,
    //    &mut clocks,
    //    peripherals.SERCOM4,
    //    &mut peripherals.MCLK,
    //    peripherals.TC2,
    //    &mut delay,
    //    &mut pins.port,
    //)
    //.unwrap();

    //let mut pins = Pins::new(peripherals.PORT).split();
    //let mut adc1 = hal::adc::Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut joystick = pins.joystick.init(&mut pins.port);

    let mut fb = FrameBuffer::new();
    let mut console = heapless::String::<256>::new();

    let mut buttons = pins.buttons.init(&mut pins.port);

    let i2c = pins.i2c.init(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut pins.port,
    );

    let mut lis3dh = Lis3dh::new_i2c(i2c, lis3dh::SlaveAddr::Alternate).unwrap();
    lis3dh.set_range(lis3dh::Range::G2).unwrap();
    lis3dh.set_datarate(lis3dh::DataRate::Hz_100).unwrap();

    let mut adc1 = Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut light = pins.light_pin.into_function_b(&mut pins.port);

    let mut frame = 0;
    loop {
        console.clear();

        frame += 1;
        writeln!(&mut console, "Frame: {frame}").unwrap();

        let (x, y) = joystick.read(&mut adc1);
        writeln!(&mut console, "Joystick: {x} {y}").unwrap();

        let F32x3 { x, y, z } = lis3dh.accel_norm().unwrap();
        writeln!(&mut console, "gx {x:+0.3}").unwrap();
        writeln!(&mut console, "gy {y:+0.3}").unwrap();
        writeln!(&mut console, "gz {z:+0.3}").unwrap();

        let light_data: u16 = adc1.read(&mut light).unwrap();
        writeln!(&mut console, "light {}", light_data).unwrap();

        for event in buttons.events() {
            writeln!(&mut console, "{:?}", event).unwrap();
        }

        let text = Text::new(&console, Point::new(1, 9), text_style());

        fb.clear(Rgb565::BLACK).unwrap();
        text.draw(&mut fb).unwrap();

        //upload(&fb, &mut display).unwrap();

        display
            .set_address_window(0, 0, SCREEN_W as u16, SCREEN_H as u16)
            .map_err(my_err)
            .unwrap();
        display
            .write_pixels(fb.inner.iter().map(|c| c.into_storage()))
            .map_err(my_err)
            .unwrap();
    }
}

#[derive(Debug)]
struct MyErr {}

// fn main_loop(display: &mut Display) -> Result<(), MyErr> {
//     let mut fb = FrameBuffer::new();
//
//     let mut console = heapless::String::<32>::new();
//
//     let mut frame = 0;
//     loop {
//         console.clear();
//
//         writeln!(&mut console, "{frame}").unwrap();
//         let text = Text::new(&console, Point::new(0, 10), text_style());
//
//         fb.clear(Rgb565::BLACK).unwrap();
//         text.draw(&mut fb).unwrap();
//
//         upload(&fb, display)?;
//     }
//
//     //let raw_image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("../ferris.bmp")).unwrap();
//     //let ferris = Image::new(&raw_image, Point::new(32, 32));
//     //ferris.draw(&mut display).unwrap();
//
//     Ok(())
// }

//  fn upload(src: &FrameBuffer, dst: &mut Display) -> Result<(), MyErr> {
//      //dst.draw_iter(src.iter_pixels()).unwrap();
//      //let N = SCREEN_H * SCREEN_W / 2;
//
//      dst.set_address_window(0, 0, SCREEN_W as u16, SCREEN_H as u16)
//          .map_err(my_err)?;
//      dst.write_pixels(src.inner.iter().map(|c| c.into_storage()))
//          .map_err(my_err)?;
//      Ok(())
//  }

fn my_err<E>(_e: E) -> MyErr {
    MyErr {}
}

fn text_style() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&mono_font::ascii::FONT_7X13_BOLD, Rgb565::WHITE)
}

//fn clear(display: &mut Display) {
//    unwrap(display.clear(Rgb565::BLACK))
//}

fn unwrap<E>(r: Result<(), E>) {
    r.map_err(|_| MyErr {}).unwrap()
}

fn print(display: &mut Display, text: &str) {
    Text::new(text, Point::new(0, 10), text_style())
        .draw(display)
        .map_err(|_| MyErr {})
        .unwrap();
}
