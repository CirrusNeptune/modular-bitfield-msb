use modular_bitfield_msb::prelude::*;

#[bitfield]
#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct SignedInt {
    sign: bool,
    value: B31,
}

fn main() {
    let i1 = SignedInt::new().with_sign(true).with_value(0b1001_0011);
    let i2 = SignedInt::from(0b1000_0000_0000_0000_0000_0000_1001_0011_u32);
    assert_eq!(i1, i2);
    assert_eq!(i1.sign(), i2.sign());
    assert_eq!(i1.value(), i2.value());
}
