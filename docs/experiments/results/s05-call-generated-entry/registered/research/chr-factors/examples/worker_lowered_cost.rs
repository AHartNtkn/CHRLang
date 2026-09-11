#[path = "../experiments/lowered_worker_cost.rs"]
mod harness;
struct Ordinary;
impl harness::Meter for Ordinary {
    type Start = ();
    fn begin() {}
    fn end(_: ()) -> Option<String> {
        None
    }
}
fn main() {
    harness::run::<Ordinary>();
}
