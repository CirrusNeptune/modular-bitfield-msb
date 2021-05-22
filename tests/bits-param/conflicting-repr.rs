use modular_bitfield_msb::prelude::*;

#[bitfield(bits = 16)]
#[repr(u32)]
pub struct SignInteger {
    sign: bool,
    value: B31,
}

fn main() {}
