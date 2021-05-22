use modular_bitfield_msb::prelude::*;
use modular_bitfield_msb::error::OutOfBounds;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Unfilled {
    a: B2,
}

fn main() {
    assert_eq!(Unfilled::from_bytes([0x00]), Ok(Unfilled::new()));
    assert_eq!(Unfilled::from_bytes([0b0100_0000]), Ok(Unfilled::new().with_a(1)));
    assert_eq!(Unfilled::from_bytes([0b1000_0000]), Ok(Unfilled::new().with_a(2)));
    assert_eq!(Unfilled::from_bytes([0b1100_0000]), Ok(Unfilled::new().with_a(3)));
    assert_eq!(Unfilled::from_bytes([0b0010_0000]), Err(OutOfBounds));
}
