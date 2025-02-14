use modular_bitfield_msb::prelude::*;

// There are 2 duplicate bytes parameters.
#[bitfield(bytes = 4, bytes = 4)]
pub struct Base {
    a: B2,
    b: B6,
    c: u8,
    d: u16,
}

fn main() {}
