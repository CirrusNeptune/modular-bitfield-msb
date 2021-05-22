use modular_bitfield_msb::prelude::*;

#[bitfield]
#[repr(u16)] // Too few bits!
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
