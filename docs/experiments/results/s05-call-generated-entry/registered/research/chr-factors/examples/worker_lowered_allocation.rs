#[path = "../experiments/lowered_worker_cost.rs"]
mod harness;
#[allow(dead_code, unexpected_cfgs)]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
struct Allocation;
impl harness::Meter for Allocation {
    type Start = meter::Start;
    fn begin() -> Self::Start {
        meter::begin()
    }
    fn end(start: Self::Start) -> Option<String> {
        let r = meter::end(start);
        assert_eq!(r.live_start, r.live_end);
        Some(r.json())
    }
}
fn main() {
    harness::run::<Allocation>();
}
