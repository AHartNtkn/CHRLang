use std::hint::black_box;
use std::time::Instant;
fn pair() -> u128 {
    let t = Instant::now();
    black_box(());
    t.elapsed().as_nanos()
}
fn bulk(with_clock: bool) -> u128 {
    let mut values = vec![0u128; 100_000];
    let t = Instant::now();
    for (i, value) in values.iter_mut().enumerate() {
        *value = if with_clock { pair() } else { black_box(i as u128) };
    }
    let elapsed = t.elapsed().as_nanos();
    if !with_clock {
        assert!(values.iter().enumerate().all(|(i, &v)| v == i as u128));
    }
    black_box(values);
    elapsed
}
unsafe extern "C" { fn sched_getcpu() -> i32; }
fn main() {
    let order: u8 = std::env::args().nth(1).unwrap().parse().unwrap();
    for _ in 0..1000 { black_box(pair()); }
    let samples: Vec<_> = (0..10_000).map(|_| pair()).collect();
    let (baseline, measured) = if order == 0 { (bulk(false), bulk(true)) } else { let a = bulk(true); (bulk(false), a) };
    let cpu = unsafe { sched_getcpu() };
    println!("{{\"cpu\":{cpu},\"baseline_ns\":{baseline},\"pairs_ns\":{measured},\"baseline_verified\":true,\"samples\":{samples:?}}}");
}
