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

fn text_style() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&mono_font::ascii::FONT_7X13_BOLD, Rgb565::BLUE)
}
