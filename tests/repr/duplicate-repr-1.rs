use modular_bitfield_msb::prelude::*;

#[bitfield]
#[repr(u32)]
#[repr(u32)]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
