//! Isolated standard-library ownership probe; executes no CHR source or engine.
//! A live sender and an empty channel force a blocking timed receive, then the
//! same channel delivers a value. All channel handles are dropped per cycle.
#[path = "../../chr-persistent/examples/support/allocator.rs"]
mod allocator;
#[global_allocator]
static ALLOCATOR: allocator::Meter = allocator::Meter;
fn cycle() {
    let (sender, receiver) = std::sync::mpsc::sync_channel::<()>(1);
    assert_eq!(
        receiver.recv_timeout(std::time::Duration::from_millis(1)),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout)
    );
    sender.send(()).unwrap();
    receiver.recv().unwrap();
    drop(sender);
    drop(receiver);
}
fn main() {
    let baseline = allocator::start();
    std::thread::spawn(cycle).join().unwrap();
    let worker = allocator::read();
    cycle();
    let first = allocator::read();
    cycle();
    let second = allocator::read();
    println!(
        "{{\"baseline_live\":{},\"worker_receiver_after_join_live\":{},\"main_receiver_first_live\":{},\"main_receiver_second_live\":{},\"allocation_calls\":{},\"requested_bytes\":{},\"peak_live\":{}}}",
        baseline.live,
        worker.live,
        first.live,
        second.live,
        second.calls - baseline.calls,
        second.requested - baseline.requested,
        second.peak
    );
}
