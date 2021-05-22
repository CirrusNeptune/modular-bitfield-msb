use modular_bitfield_msb::prelude::*;

#[derive(BitfieldSpecifier)]
pub union InvalidUnionSpecifier {
    a: bool,
    b: B7,
    c: u8,
}

fn main() {}
