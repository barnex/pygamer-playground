use crate::lib::types::*;

pub struct Sys {
    pub hw: HW,
}

impl Sys {
    pub fn new() -> Self {
        Self { hw: HW::new() }
    }
}
