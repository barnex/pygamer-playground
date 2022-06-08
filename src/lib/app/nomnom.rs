use crate::lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

use bsp::buttons::Keys;

use embedded_graphics as eg;

use eg::draw_target::DrawTarget;
use eg::image::Image;
use eg::pixelcolor::Rgb565;
use eg::text::Text;

use tinybmp::Bmp;

pub fn main(sys: &mut Sys) {
    let raw_image: Bmp<Rgb565> =
        Bmp::from_slice(include_bytes!("../../../assets/nomnom64.bmp")).unwrap();

    let mut pos = (0.0f32, 0.0f32);
    let mut vel = (0.0f32, 0.0f32);
    const SIZE: i16 = 64;
    const W: i16 = SCREEN_W as i16;
    const H: i16 = SCREEN_H as i16;

    let mut dbg = false;

    sys.fb.clear(Rgb565::WHITE).unwrap();
    let text = Text::new("Hello...", Point::new(30, 60), text_style());
    text.draw(&mut sys.fb).unwrap();
    sys.present_fb();

    sys.wait_for_key();

    sys.fb.clear(Rgb565::WHITE).unwrap();
    let text = Text::new("Wanna play?", Point::new(30, 60), text_style());
    text.draw(&mut sys.fb).unwrap();
    sys.present_fb();

    sys.wait_for_key();

    loop {
        sys.fb.clear(Rgb565::WHITE).unwrap();

        for event in sys.button_events() {
            match event {
                Keys::SelectDown => dbg = !dbg,
                _ => (),
            }
        }

        let (gx, gy, gz) = sys.accel_read();

        let gx = gx as f32 / 16368.0;
        let gy = gy as f32 / 16368.0;
        let gz = gz as f32 / 16368.0;

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
        nomnom.draw(&mut sys.fb).unwrap();

        sys.present_fb();
    }
}
