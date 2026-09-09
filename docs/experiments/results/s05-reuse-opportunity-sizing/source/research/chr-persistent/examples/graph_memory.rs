#[path = "support/allocator.rs"]
mod allocator;
#[path = "support/graph_cost_cases.rs"]
#[allow(dead_code)]
mod graph_cost_cases;
#[path = "support/graph_session.rs"]
mod graph_session;
#[path = "support/graph_cost.rs"]
mod harness;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
struct Metered;
fn reading(r: allocator::Reading) -> harness::Reading {
    harness::Reading {
        calls: r.calls,
        requested: r.requested,
        live: r.live,
        peak: r.peak,
    }
}
impl harness::Metering for Metered {
    const ENABLED: bool = true;
    fn start() -> harness::Reading {
        reading(allocator::start())
    }
    fn read() -> harness::Reading {
        reading(allocator::read())
    }
}
fn main() {
    harness::main::<Metered>();
}
