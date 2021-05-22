use modular_bitfield_msb::prelude::*;

#[bitfield(bits = true)]
pub struct SignInteger {
    sign: bool,
    value: B31,
}

fn main() {}
