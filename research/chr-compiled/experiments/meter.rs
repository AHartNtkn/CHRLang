//! Requested-allocation accounting for isolated single-thread measurement processes.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static CALLS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static FREES: AtomicUsize = AtomicUsize::new(0);
struct Meter;
#[global_allocator]
static ALLOCATOR: Meter = Meter;
fn allocated(size: usize) {
    CALLS.fetch_add(1, Relaxed);
    BYTES.fetch_add(size, Relaxed);
    let live = LIVE.fetch_add(size, Relaxed) + size;
    PEAK.fetch_max(live, Relaxed);
}
unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            allocated(layout.size());
        }
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            allocated(layout.size());
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        LIVE.fetch_sub(layout.size(), Relaxed);
        FREES.fetch_add(1, Relaxed);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let next = unsafe { System.realloc(ptr, layout, new_size) };
        if !next.is_null() {
            CALLS.fetch_add(1, Relaxed);
            BYTES.fetch_add(new_size, Relaxed);
            let live = if new_size >= layout.size() {
                LIVE.fetch_add(new_size - layout.size(), Relaxed) + new_size - layout.size()
            } else {
                LIVE.fetch_sub(layout.size() - new_size, Relaxed) - (layout.size() - new_size)
            };
            PEAK.fetch_max(live, Relaxed);
        }
        next
    }
}
#[derive(Clone, Copy)]
pub struct Start {
    live: usize,
    calls: usize,
    bytes: usize,
    frees: usize,
}
#[derive(Clone, Copy)]
pub struct Reading {
    pub live_start: usize,
    pub live_end: usize,
    pub peak_live: usize,
    pub allocation_calls: usize,
    pub requested_bytes: usize,
    pub deallocation_calls: usize,
}
pub fn begin() -> Start {
    let live = LIVE.load(Relaxed);
    PEAK.store(live, Relaxed);
    Start {
        live,
        calls: CALLS.load(Relaxed),
        bytes: BYTES.load(Relaxed),
        frees: FREES.load(Relaxed),
    }
}
pub fn end(start: Start) -> Reading {
    Reading {
        live_start: start.live,
        live_end: LIVE.load(Relaxed),
        peak_live: PEAK.load(Relaxed),
        allocation_calls: CALLS.load(Relaxed) - start.calls,
        requested_bytes: BYTES.load(Relaxed) - start.bytes,
        deallocation_calls: FREES.load(Relaxed) - start.frees,
    }
}
impl Reading {
    pub fn json(self) -> String {
        format!(
            "{{\"live_start\":{},\"live_end\":{},\"peak_live\":{},\"allocation_calls\":{},\"requested_bytes\":{},\"deallocation_calls\":{}}}",
            self.live_start,
            self.live_end,
            self.peak_live,
            self.allocation_calls,
            self.requested_bytes,
            self.deallocation_calls
        )
    }
}
/// Run only as a separate process: unrelated test threads would contaminate exact gauges.
pub fn self_check() -> Result<(), String> {
    let start = begin();
    let buffer = vec![0u8; 1024];
    std::hint::black_box(&buffer);
    let allocated = end(start);
    if allocated.live_end != start.live + 1024 || allocated.requested_bytes != 1024 {
        return Err("zeroed allocation was not accounted exactly".into());
    }
    drop(buffer);
    if end(start).live_end != start.live {
        return Err("deallocation was not accounted".into());
    }
    let start = begin();
    let mut growing = Vec::<u8>::with_capacity(16);
    growing.extend(0..64);
    std::hint::black_box(&growing);
    let held = end(start);
    if held.live_end != start.live + growing.capacity() || held.allocation_calls < 2 {
        return Err("reallocation was not accounted".into());
    }
    growing.truncate(8);
    growing.shrink_to_fit();
    if end(start).live_end != start.live + growing.capacity() {
        return Err("shrinking allocation was not accounted".into());
    }
    drop(growing);
    if end(start).live_end != start.live {
        return Err("reallocation teardown leaked requested bytes".into());
    }
    #[cfg(feature = "fork-diagnostics")]
    {
        let outer = begin();
        let before = checkpoint();
        let buffer = vec![0_u8; 128];
        std::hint::black_box(&buffer);
        let allocated = checkpoint();
        let peak = end(outer).peak_live;
        drop(buffer);
        let freed = checkpoint();
        if allocated.requested_bytes - before.requested_bytes != 128
            || allocated.allocation_calls - before.allocation_calls != 1
            || freed.deallocation_calls - allocated.deallocation_calls != 1
            || end(outer).peak_live != peak
        {
            return Err("cumulative checkpoint altered peak or miscounted traffic".into());
        }
    }
    Ok(())
}

/// Cumulative allocation traffic; reading never resets the active peak window.
#[cfg(feature = "fork-diagnostics")]
#[derive(Clone, Copy, Default)]
pub struct Checkpoint {
    pub allocation_calls: usize,
    pub requested_bytes: usize,
    pub deallocation_calls: usize,
}
#[cfg(feature = "fork-diagnostics")]
pub fn checkpoint() -> Checkpoint {
    Checkpoint {
        allocation_calls: CALLS.load(Relaxed),
        requested_bytes: BYTES.load(Relaxed),
        deallocation_calls: FREES.load(Relaxed),
    }
}
