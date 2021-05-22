// Checks that no implicit paths are generated by the `#[bitfield]` proc. macro
// and `#[derive(BitfieldSpecifier)]` derive macro.

#![no_implicit_prelude]

use ::modular_bitfield_msb::prelude::*;

#[bitfield]
#[derive(BitfieldSpecifier, Debug)]
pub struct TestBitfield {
    a: ::core::primitive::bool,
    b: ::modular_bitfield_msb::specifiers::B3,
    c: ::modular_bitfield_msb::specifiers::B4,
    d: ::modular_bitfield_msb::specifiers::B24,
}

fn main() {}
