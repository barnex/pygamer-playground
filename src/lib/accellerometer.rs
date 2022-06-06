use bsp::{entry, hal, pac, Pins};
use pygamer as bsp;

use lis3dh::*;
use pygamer::gpio::v1 as gpio;
use pygamer::gpio::v2::Alternate;
use pygamer::gpio::v2::C;
use pygamer::gpio::v2::PA12;
use pygamer::gpio::v2::PA13;
use pygamer::sercom::v1::Pad;
use pygamer::sercom::v2::Pad0;
use pygamer::sercom::v2::Pad1;
use pygamer::sercom::I2CMaster2;

pub type AccMtr = Lis3dh<
    Lis3dhI2C<
        I2CMaster2<
            Pad<pac::SERCOM2, Pad0, gpio::Pin<PA12, Alternate<C>>>,
            Pad<pac::SERCOM2, Pad1, gpio::Pin<PA13, Alternate<C>>>,
        >,
    >,
>;
