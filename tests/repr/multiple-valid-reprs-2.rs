use modular_bitfield_msb::prelude::*;

#[bitfield]
#[repr(u32)]
#[repr(C)] // The macro simply ignores `repr(C)`
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
