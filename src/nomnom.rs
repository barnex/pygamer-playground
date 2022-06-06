#![no_std]
#![no_main]

mod lib;
use lib::accellerometer::*;
use lib::display::*;
use lib::framebuffer::*;

use bsp::buttons::ButtonReader;
use bsp::pins::JoystickReader;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

use core::fmt::Write;

use pac::gclk::pchctrl::GEN_A::GCLK11;

use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio;
use hal::time::KiloHertz;

use pygamer as bsp;

use bsp::buttons::Keys;
use bsp::gpio::v2::Alternate;
use bsp::gpio::v2::B;
use bsp::gpio::v2::PB04;
use bsp::prelude::*;
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
use eg::text::Text;

use tinybmp::Bmp;

// Hardware state
struct HW {
    pub delay: Delay,
    pub display: Display,
    pub joystick: JoystickReader,
    pub buttons: ButtonReader,
    pub lis3dh: AccMtr,
    pub adc1: Adc<pac::ADC1>,
    pub light: gpio::Pin<PB04, Alternate<B>>,
    pub fb: FrameBuffer,
}

impl HW {
    pub fn new() -> Self {
        let mut peripherals = pac::Peripherals::take().unwrap();
        let mut pins = Pins::new(peripherals.PORT).split();

        let core = pac::CorePeripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut delay = Delay::new(core.SYST, &mut clocks);

        let (display, _backlight) = pins
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

        let joystick = pins.joystick.init(&mut pins.port);

        let buttons = pins.buttons.init(&mut pins.port);

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

        let adc1 = Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
        let light = pins.light_pin.into_function_b(&mut pins.port);

        let mut fb = FrameBuffer::new();

        HW {
            delay,
            display,
            joystick,
            buttons,
            lis3dh,
            adc1,
            light,
            fb,
        }
    }

    // Copy the framebuffer to the display.
    pub fn present_fb(&mut self) {
        self.display
            .set_address_window(0, 0, SCREEN_W as u16, SCREEN_H as u16)
            .map_err(my_err)
            .unwrap();
        self.display
            .write_pixels(self.fb.inner.iter().map(|c| c.into_storage()))
            .map_err(my_err)
            .unwrap();
    }
}

#[entry]
fn main() -> ! {
    let mut hw = HW::new();

    let mut console = heapless::String::<256>::new();
    let mut frame = 0;

    let raw_image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("../assets/nomnom64.bmp")).unwrap();

    let mut pos = (0.0f32, 0.0f32);
    let mut vel = (0.0f32, 0.0f32);
    const SIZE: i16 = 64;
    const W: i16 = SCREEN_W as i16;
    const H: i16 = SCREEN_H as i16;

    let mut dbg = false;

    hw.fb.clear(Rgb565::WHITE).unwrap();
    let text = Text::new("Hello...", Point::new(30, 60), text_style());
    text.draw(&mut hw.fb).unwrap();
    //upload(&hw.fb, &mut hw.display);
    hw.present_fb();

    'wait: loop {
        for event in hw.buttons.events() {
            match event {
                Keys::ADown | Keys::BDown => break 'wait,
                _ => (),
            }
        }
    }

    hw.fb.clear(Rgb565::WHITE).unwrap();
    let text = Text::new("Wanna play?", Point::new(30, 60), text_style());
    text.draw(&mut hw.fb).unwrap();
    //upload(&hw.fb, &mut hw.display);
    hw.present_fb();

    hw.delay.delay_ms(500u16);

    'wait2: loop {
        for event in hw.buttons.events() {
            match event {
                Keys::ADown | Keys::BDown => break 'wait2,
                _ => (),
            }
        }
    }

    loop {
        hw.fb.clear(Rgb565::WHITE).unwrap();

        for event in hw.buttons.events() {
            match event {
                Keys::SelectDown => dbg = !dbg,
                _ => (),
            }
        }

        let F32x3 {
            x: gx,
            y: gy,
            z: gz,
        } = hw.lis3dh.accel_norm().unwrap();

        const ACC: f32 = 0.08;
        vel.0 += ACC * gx;
        vel.1 += ACC * gy;

        pos.0 += vel.0;
        pos.1 += vel.1;

        let ipos = (pos.0 as i16, pos.1 as i16);

        if ipos.0 < 0 {
            pos.0 = 0.0;
            vel.0 = -vel.0 / 2.0;
        }

        if ipos.1 < 0 {
            pos.1 = 0.0;
            vel.1 = -vel.1 / 2.0;
        }

        if ipos.0 >= (W - SIZE) {
            pos.0 = (W - SIZE - 1) as f32;
            vel.0 = -vel.0 / 2.0;
        }

        if ipos.1 >= (H - SIZE) {
            pos.1 = (H - SIZE - 1) as f32;
            vel.1 = -vel.1 / 2.0;
        }

        let nomnom = Image::new(&raw_image, Point::new(pos.0 as i32, pos.1 as i32));
        nomnom.draw(&mut hw.fb).unwrap();

        if dbg {
            console.clear();
            frame += 1;

            writeln!(&mut console, "Frame: {frame}").unwrap();

            let (x, y) = hw.joystick.read(&mut hw.adc1);
            writeln!(&mut console, "Joystick: {x} {y}").unwrap();

            writeln!(&mut console, "gx {gx:+0.3}").unwrap();
            writeln!(&mut console, "gy {gy:+0.3}").unwrap();
            writeln!(&mut console, "gz {gz:+0.3}").unwrap();

            let light_data: u16 = hw.adc1.read(&mut hw.light).unwrap();
            writeln!(&mut console, "light {}", light_data).unwrap();

            let text = Text::new(&console, Point::new(1, 9), text_style());
            text.draw(&mut hw.fb).unwrap();
        }

        //upload(&hw.fb, &mut hw.display);
        hw.present_fb();
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
    MonoTextStyle::new(&mono_font::ascii::FONT_7X13_BOLD, Rgb565::BLUE)
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
