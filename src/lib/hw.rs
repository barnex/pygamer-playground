use crate::lib::types::*;

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

use bsp::gpio::v2::C;
use bsp::gpio::v2::PA12;
use bsp::gpio::v2::PA13;
use bsp::sercom::v1::Pad;
use bsp::sercom::v2::Pad0;
use bsp::sercom::v2::Pad1;
use bsp::sercom::I2CMaster2;
use lis3dh::*;

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
}

pub type AccMtr = Lis3dh<
    Lis3dhI2C<
        I2CMaster2<
            Pad<pac::SERCOM2, Pad0, gpio::Pin<PA12, Alternate<C>>>,
            Pad<pac::SERCOM2, Pad1, gpio::Pin<PA13, Alternate<C>>>,
        >,
    >,
>;

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

        HW {
            delay,
            display,
            joystick,
            buttons,
            lis3dh,
            adc1,
            light,
        }
    }

}

pub fn text_style() -> MonoTextStyle<'static, Rgb565> {
    MonoTextStyle::new(&eg::mono_font::ascii::FONT_7X13_BOLD, Rgb565::BLUE)
}
