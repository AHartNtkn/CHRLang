#[cfg(all(feature = "head-dispatch", feature = "alloc-meter"))]
include!("support/finite_service_probe.rs");
#[cfg(not(all(feature = "head-dispatch", feature = "alloc-meter")))]
fn main() {
    panic!("probe requires head-dispatch and alloc-meter")
}
