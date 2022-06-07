use crate::lib::types::*;
use crate::LINE_H;

use crate::lib::accellerometer::*;

use pac::gclk::pchctrl::GEN_A::GCLK11;

use hal::adc::Adc;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio;
use hal::time::KiloHertz;

use pygamer as bsp;

use bsp::buttons::ButtonReader;
use bsp::gpio::v2::Alternate;
use bsp::gpio::v2::B;
use bsp::gpio::v2::PB04;
use bsp::pins::JoystickReader;
use bsp::{hal, pac, Pins};

use lis3dh::Lis3dh;

// Hardware state
pub struct HW {
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

        let fb = FrameBuffer::new();

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
            .unwrap();
        self.display
            .write_pixels(self.fb.inner.iter().map(|c| c.into_storage()))
            .unwrap();
    }

    pub fn joystick_read(&mut self) -> (i16, i16) {
        let (x, y) = self.joystick.read(&mut self.adc1);
        (x as i16 - 2048, y as i16 - 2048)
    }

    pub fn button_pressed(&mut self, button: Keys) -> bool {
        self.buttons.events().find(|b| *b == button).is_some()
    }

    pub fn wait_for_key(&mut self) {
        'wait: loop {
            for event in self.buttons.events() {
                match event {
                    Keys::ADown | Keys::BDown => break 'wait,
                    _ => (),
                }
            }
        }
    }

    pub fn show_msg(&mut self, msg: &str) {
        self.fb.clear(Rgb565::WHITE).unwrap();
        Text::new(msg, Point::new(0, LINE_H as i32 - 2), text_style())
            .draw(&mut self.fb)
            .unwrap();
        self.present_fb();
    }
}

pub fn text_style() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&eg::mono_font::ascii::FONT_7X13_BOLD, Rgb565::BLUE)
}
