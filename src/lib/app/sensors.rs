//! An app that displays sensors / inputs readouts.

use crate::lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

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

        let (gx, gy, gz) = sys.accel_read();
        writeln!(&mut console, "gx {gx:+}").unwrap();
        writeln!(&mut console, "gy {gy:+}").unwrap();
        writeln!(&mut console, "gz {gz:+}").unwrap();

        writeln!(&mut console, "light {}", sys.light_read()).unwrap();

        let text = Text::new(&console, Point::new(1, 9), text_style());
        text.draw(&mut sys.fb).unwrap();

        sys.present_fb();
    }
}
