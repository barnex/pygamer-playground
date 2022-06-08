//! An app that displays sensors / inputs readouts.

use crate::lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

use lis3dh::accelerometer::vector::I16x3;
use lis3dh::accelerometer::RawAccelerometer;

use embedded_graphics as eg;

use eg::mono_font;
use eg::mono_font::MonoTextStyle;
use eg::text::Text;

pub fn main(hw: &mut HW) {
    let mut console = heapless::String::<256>::new();
    let mut frame = 0;
    loop {
        hw.fb.clear(Rgb565::WHITE).unwrap();
        console.clear();
        frame += 1;

        writeln!(&mut console, "Frame: {frame}").unwrap();

        let (x, y) = hw.joystick.read(&mut hw.adc1);
        writeln!(&mut console, "Joystick: {x} {y}").unwrap();

        let I16x3 {
            x: gx,
            y: gy,
            z: gz,
        } = hw.lis3dh.accel_raw().unwrap();
        writeln!(&mut console, "gx {gx}").unwrap();
        writeln!(&mut console, "gy {gy}").unwrap();
        writeln!(&mut console, "gz {gz}").unwrap();

        let light_data: u16 = hw.adc1.read(&mut hw.light).unwrap();
        writeln!(&mut console, "light {}", light_data).unwrap();

        let text = Text::new(&console, Point::new(1, 9), text_style());
        text.draw(&mut hw.fb).unwrap();

        hw.present_fb();
    }
}

fn text_style() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&mono_font::ascii::FONT_7X13_BOLD, Rgb565::BLUE)
}
