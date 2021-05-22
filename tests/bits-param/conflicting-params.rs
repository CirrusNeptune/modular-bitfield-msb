use modular_bitfield_msb::prelude::*;

#[bitfield(bits = 16, bytes = 4)]
pub struct SignInteger {
    sign: bool,
    value: B31,
}

fn main() {}
