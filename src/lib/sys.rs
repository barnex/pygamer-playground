use crate::lib::types::*;

pub struct Sys {
    pub hw: HW,
}

impl Sys {
    pub const BG: Rgb565 = Rgb565::WHITE;
    pub const FG: Rgb565 = Rgb565::BLUE;

    pub fn new() -> Self {
        Self { hw: HW::new() }
    }

    pub fn joystick_read(&mut self) -> (i16, i16) {
        let (x, y) = self.hw.joystick.read(&mut self.hw.adc1);
        (x as i16 - 2048, y as i16 - 2048)
    }

    pub fn show_menu(self: &mut Self, opts: &[&str]) -> usize {
        let mut sel: i32 = 0;
        loop {
            let mut must_sleep = false;
            self.hw.fb.clear(Sys::BG).unwrap();

            let joy_y = self.joystick_read().1;
            if joy_y < -1400 {
                sel -= 1;
                must_sleep = true;
            }
            if joy_y > 1400 {
                sel += 1;
                must_sleep = true;
            }
            if sel < 0 {
                sel = opts.len() as i32 - 1;
            }
            if sel >= opts.len() as i32 {
                sel = 0;
            }

            if self.hw.button_pressed(Keys::ADown) {
                return sel as usize;
            }

            for (i, opt) in opts.iter().enumerate() {
                let i = i as i32;
                let x = 1;
                let y = (i + 1) as i32 * LINE_H as i32 - 3;
                let style = if i == sel { sel_style() } else { text_style() };
                if i == sel {
                    self.hw.fb
                        .fill_solid(
                            &eg::primitives::Rectangle::new(
                                eg::geometry::Point {
                                    x,
                                    y: y - LINE_H as i32 + 4,
                                },
                                eg::geometry::Size {
                                    width: SCREEN_W as u32 - 2,
                                    height: LINE_H as u32,
                                },
                            ),
                            Rgb565::BLUE,
                        )
                        .unwrap();
                }
                Text::new(opt, Point::new(x + 1, y), style)
                    .draw(&mut self.hw.fb)
                    .unwrap();
            }

            self.hw.present_fb();
            if must_sleep {
                for _tick in 0..16 {
                    if self.joystick_read().1.abs() < 800 {
                        break;
                    }
                    self.hw.delay.delay_ms(10u16);
                }
            }
        }
    }
}

pub fn sel_style() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&eg::mono_font::ascii::FONT_7X13_BOLD, Rgb565::WHITE)
}

pub const LINE_H: u16 = 14;
