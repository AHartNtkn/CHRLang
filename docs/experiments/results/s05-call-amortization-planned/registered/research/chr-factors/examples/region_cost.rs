//! Ordinary-System regional lifecycle timing. No allocation meter is linked.
#[path = "support/region_cost.rs"]
mod harness;
#[path = "support/region_cost_cases.rs"]
mod region_cost_cases;
#[global_allocator]
static ALLOCATOR: std::alloc::System = std::alloc::System;

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
