//! An app that displays sensors / inputs readouts.

use crate::lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

use lis3dh::accelerometer::vector::I16x3;
use lis3dh::accelerometer::RawAccelerometer;

use embedded_graphics as eg;

use eg::text::Text;

pub fn main(sys: &mut Sys) {
    let mut console = heapless::String::<256>::new();
    let mut frame = 0;
    loop {
        sys.fb.clear(Sys::BG).unwrap();
        console.clear();
        frame += 1;

        writeln!(&mut console, "Frame: {frame}").unwrap();

        let (x, y) = sys.joystick_read();
        writeln!(&mut console, "Joystick: {x} {y}").unwrap();

        let I16x3 {
            x: gx,
            y: gy,
            z: gz,
        } = sys.hw.lis3dh.accel_raw().unwrap();
        writeln!(&mut console, "gx {gx:+}").unwrap();
        writeln!(&mut console, "gy {gy:+}").unwrap();
        writeln!(&mut console, "gz {gz:+}").unwrap();

        let light_data: u16 = sys.hw.adc1.read(&mut sys.hw.light).unwrap();
        writeln!(&mut console, "light {}", light_data).unwrap();

        let text = Text::new(&console, Point::new(1, 9), text_style());
        text.draw(&mut sys.fb).unwrap();

        sys.present_fb();
    }
}
