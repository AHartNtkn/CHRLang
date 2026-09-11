#[cfg(feature = "head-dispatch")]
include!("support/effect_lifecycle.rs");
#[cfg(not(feature = "head-dispatch"))]
fn main() {
    panic!("effect lifecycle requires head-dispatch");
}
