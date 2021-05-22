use modular_bitfield_msb::prelude::*;

#[bitfield]
#[derive(BitfieldSpecifier)]
pub struct Header {
    live: bool,
    received: bool,
    status: B2,
    rest: B4,
}

fn main() {
    assert_eq!(<Header as Specifier>::BITS, 8);
}
