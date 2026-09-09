//! Ordinary allocator, contiguous cold regional lifecycle.
#[path = "support/region_lifecycle.rs"]
mod harness;
struct Unmetered;
impl harness::Metering for Unmetered {
    const ENABLED: bool = false;
    fn start() -> harness::Reading {
        harness::Reading::default()
    }
    fn read() -> harness::Reading {
        harness::Reading::default()
    }
}
fn main() {
    harness::main::<Unmetered>();
}
