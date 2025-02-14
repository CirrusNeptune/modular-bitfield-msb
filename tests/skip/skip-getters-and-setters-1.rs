use modular_bitfield_msb::prelude::*;

#[bitfield]
pub struct Sparse {
    #[skip(getters, setters)]
    unused_1: B10,
    a: bool,
    #[skip(getters, setters)]
    unused_2: B10,
    b: bool,
    #[skip(getters, setters)]
    unused_3: B10,
}

fn main() {
    let mut sparse = Sparse::new();
    assert!(!sparse.a());
    assert!(!sparse.b());
    sparse.set_a(true);
    sparse.set_b(true);
    assert!(sparse.a());
    assert!(sparse.b());
}
