//! Separate thread-safe allocation run; elapsed times are diagnostic only.
#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[path = "support/region_lifecycle.rs"]
mod harness;
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
    if std::env::args().nth(1).as_deref() == Some("--meter-check") {
        let before = allocator::start();
        let buffer = vec![0u8; 1024];
        std::hint::black_box(&buffer);
        let held = allocator::read();
        assert_eq!(held.live, before.live + 1024);
        assert_eq!(held.requested, before.requested + 1024);
        drop(buffer);
        assert_eq!(allocator::read().live, before.live);
        println!("{{\"meter_check\":true}}");
        return;
    }
    harness::main::<Metered>();
}
