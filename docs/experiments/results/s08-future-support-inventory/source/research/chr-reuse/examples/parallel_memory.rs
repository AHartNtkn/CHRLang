//! E16 separately metered allocation executable. Its times are diagnostic only.
#[path = "support/allocator.rs"]
mod allocator;
#[path = "support/parallel_cost.rs"]
mod harness;
#[path = "support/parallel_cases.rs"]
mod parallel_cases;

#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;

fn reading(value: allocator::Reading) -> harness::Reading {
    harness::Reading {
        calls: value.calls,
        requested: value.requested,
        live: value.live,
        peak: value.peak,
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
