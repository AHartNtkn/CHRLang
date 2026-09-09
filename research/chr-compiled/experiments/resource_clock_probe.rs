use std::time::Instant;
#[repr(C)]
struct Timespec {
    seconds: i64,
    nanos: i64,
}
unsafe extern "C" {
    fn clock_gettime(clock: i32, time: *mut Timespec) -> i32;
}
fn cpu() -> u128 {
    let mut t = Timespec {
        seconds: 0,
        nanos: 0,
    };
    assert_eq!(unsafe { clock_gettime(2, &mut t) }, 0);
    t.seconds as u128 * 1_000_000_000 + t.nanos as u128
}
fn main() {
    let mut rows = Vec::with_capacity(5000);
    for _ in 0..5000 {
        let c = cpu();
        let t = Instant::now();
        std::hint::black_box(());
        let ns = t.elapsed().as_nanos();
        let cpu_ns = cpu() - c;
        rows.push((ns, cpu_ns));
    }
    for (ns, cpu_ns) in rows {
        println!("{ns},{cpu_ns}");
    }
}
