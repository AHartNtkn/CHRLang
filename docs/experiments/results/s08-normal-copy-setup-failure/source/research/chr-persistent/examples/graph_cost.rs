#[path = "support/graph_cost_cases.rs"]
#[allow(dead_code)]
mod graph_cost_cases;
#[path = "support/graph_session.rs"]
mod graph_session;
#[path = "support/graph_cost.rs"]
mod harness;
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
