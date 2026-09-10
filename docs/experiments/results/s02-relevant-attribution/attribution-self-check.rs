//! Standalone, single-thread check of known nested requested allocations.
#![allow(dead_code, unexpected_cfgs)]
#[path = "../../../../research/chr-compiled/experiments/meter.rs"]
mod meter;
#[path = "../../../../research/chr-relational/examples/support/deduction_profile.rs"]
mod profile;
use chr_relational::deduction_profile::{Phase, Scope};
fn main() {
    profile::enable();
    let start = meter::begin();
    {
        let _parent = Scope::new(Phase::Equality);
        let a = vec![1u8; 64];
        std::hint::black_box(&a);
        {
            let _child = Scope::new(Phase::Key);
            let b = vec![2u8; 128];
            std::hint::black_box(&b);
            {
                let _grandchild = Scope::new(Phase::RelevantReplay);
                let c = vec![3u8; 256];
                std::hint::black_box(&c);
            }
        }
        {
            let _sibling = Scope::new(Phase::Key);
            let d = vec![4u8; 512];
            std::hint::black_box(&d);
        }
    }
    let result = meter::end(start);
    assert_eq!(result.requested_bytes, 960);
    assert_eq!(result.allocation_calls, 4);
    assert_eq!(result.deallocation_calls, 4);
    assert_eq!(result.live_end, result.live_start);
    assert_eq!(result.peak_live - result.live_start, 576);
    println!("{}", profile::json());
    println!("{}", result.json());
}
