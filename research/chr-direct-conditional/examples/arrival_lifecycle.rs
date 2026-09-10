#[cfg(feature = "head-dispatch")]
include!("support/arrival_lifecycle.rs");
#[cfg(not(feature = "head-dispatch"))]
fn main() {
    panic!("arrival lifecycle requires head-dispatch");
}
