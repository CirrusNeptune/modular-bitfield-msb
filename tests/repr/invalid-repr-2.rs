use modular_bitfield_msb::prelude::*;

#[bitfield]
#[cfg_attr(not(feature = "unknown"), repr(invalid))]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {}
