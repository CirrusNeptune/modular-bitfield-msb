use modular_bitfield_msb::prelude::*;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier, Debug)]
pub struct Header {
    a: B1,
    b: B128,
}

fn main() {}
