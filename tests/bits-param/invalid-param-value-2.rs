use modular_bitfield_msb::prelude::*;

#[bitfield(bits = -1)]
pub struct SignInteger {
    sign: bool,
    value: B31,
}

fn main() {}
