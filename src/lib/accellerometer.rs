use crate::lib::types::*;

use bsp::gpio::v1 as gpio;
use bsp::gpio::v2::Alternate;
use bsp::gpio::v2::C;
use bsp::gpio::v2::PA12;
use bsp::gpio::v2::PA13;
use bsp::sercom::v1::Pad;
use bsp::sercom::v2::Pad0;
use bsp::sercom::v2::Pad1;
use bsp::sercom::I2CMaster2;
use lis3dh::*;

pub type AccMtr = Lis3dh<
    Lis3dhI2C<
        I2CMaster2<
            Pad<pac::SERCOM2, Pad0, gpio::Pin<PA12, Alternate<C>>>,
            Pad<pac::SERCOM2, Pad1, gpio::Pin<PA13, Alternate<C>>>,
        >,
    >,
>;
