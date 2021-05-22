use modular_bitfield_msb::prelude::*;
#[bitfield]
struct RawIdentifiers {
    r#struct: B5,
    r#bool: B3,
}

fn main() {
    let r = RawIdentifiers::new();
    let _ = r.r#struct();
    let _ = r.r#bool();
}
