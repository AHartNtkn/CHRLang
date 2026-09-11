//! E16 ordinary-System timing executable; no allocation meter is linked here.
#[path = "support/parallel_cost.rs"]
mod harness;
#[path = "support/parallel_cases.rs"]
mod parallel_cases;

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
