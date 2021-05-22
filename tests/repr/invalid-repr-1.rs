use modular_bitfield_msb::prelude::*;

#[bitfield]
#[repr(invalid)]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
