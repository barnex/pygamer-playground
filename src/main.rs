#![no_std]
#![no_main]

mod lib;
use bsp::entry;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use lib::types::*;

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut hw = HW::new();

    let selection = show_menu(&mut hw, &["foo", "bar", "baz"]);

    let mut msg = heapless::String::<256>::new();
    writeln!(&mut msg, "Thanks for choosing {selection}");
    hw.show_msg(&msg);

    loop {}

    //lib::app::nomnom::main(&mut hw);
}

fn show_menu(hw: &mut HW, opts: &[&str]) -> usize {
    let mut sel: i32 = 0;
    loop {
        hw.fb.clear(Rgb565::WHITE).unwrap();

        if hw.joystick_read().1 < -1024 {
            sel -= 1;
            hw.delay.delay_ms(300u16);
        }
        if hw.joystick_read().1 > 1024 {
            sel += 1;
            hw.delay.delay_ms(300u16);
        }
        if sel < 0 {
            sel = opts.len() as i32 - 1;
        }
        if sel >= opts.len() as i32 {
            sel = 0;
        }

        if hw.button_pressed(Keys::ADown) {
            return sel as usize;
        }

        for (i, opt) in opts.iter().enumerate() {
            let i = i as i32;
            let x = 1;
            let y = (i + 1) as i32 * LINE_H as i32;
            let style = if i == sel { sel_style() } else { text_style() };
            if i == sel {
                hw.fb
                    .fill_solid(
                        &eg::primitives::Rectangle::new(
                            eg::geometry::Point {
                                x,
                                y: y - LINE_H as i32 + 3,
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
                .draw(&mut hw.fb)
                .unwrap();
        }

        hw.present_fb();
    }
}

fn sel_style() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&eg::mono_font::ascii::FONT_7X13_BOLD, Rgb565::WHITE)
}

const LINE_H: u16 = 14;
