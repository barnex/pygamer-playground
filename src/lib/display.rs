use super::types::*;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 128;

pub const ISCREEN_W: i32 = SCREEN_W as i32;
pub const ISCREEN_H: i32 = SCREEN_H as i32;

use st7735_lcd;

pub type Display = st7735_lcd::ST7735<
    SPIMaster4<
        sercom::Pad<
            SERCOM4,
            sercom::v2::Pad2,
            gpio::Pin<gpio::v2::PB14, gpio::v2::Alternate<gpio::v2::C>>,
        >,
        sercom::Pad<
            SERCOM4,
            sercom::v2::Pad3,
            gpio::Pin<gpio::v2::PB15, gpio::v2::Alternate<gpio::v2::C>>,
        >,
        sercom::Pad<
            SERCOM4,
            sercom::v2::Pad1,
            gpio::Pin<gpio::v2::PB13, gpio::v2::Alternate<gpio::v2::C>>,
        >,
    >,
    gpio::Pin<gpio::v2::PB05, gpio::v2::Output<gpio::v2::PushPull>>,
    pygamer::gpio::Pin<gpio::v2::PA00, gpio::v2::Output<gpio::v2::PushPull>>,
>;
