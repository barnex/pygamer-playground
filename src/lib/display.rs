use super::types::*;

pub type Display = lcd::ST7735<
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
