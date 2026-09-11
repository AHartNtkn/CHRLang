//! Separate allocation run. Its elapsed times are diagnostic only.
#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[path = "support/region_cost.rs"]
mod harness;
#[path = "support/region_cost_cases.rs"]
mod region_cost_cases;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;

fn reading(r: allocator::Reading) -> harness::Reading {
    harness::Reading {
        calls: r.calls,
        requested: r.requested,
        live: r.live,
        peak: r.peak,
    }
}
struct Metered;
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
