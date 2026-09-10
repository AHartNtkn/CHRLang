//! Single-threaded cumulative allocation attribution; no event allocates.
use crate::meter;
use chr_relational::deduction_profile::{Phase, set_callback};
use std::cell::RefCell;
#[derive(Clone, Copy, Default)]
struct Counts {
    calls: usize,
    bytes: usize,
    frees: usize,
}
impl Counts {
    fn now() -> Self {
        let c = meter::checkpoint();
        Self {
            calls: c.allocation_calls,
            bytes: c.requested_bytes,
            frees: c.deallocation_calls,
        }
    }
    fn minus(self, b: Self) -> Self {
        Self {
            calls: self.calls - b.calls,
            bytes: self.bytes - b.bytes,
            frees: self.frees - b.frees,
        }
    }
    fn add(&mut self, b: Self) {
        self.calls += b.calls;
        self.bytes += b.bytes;
        self.frees += b.frees;
    }
}
#[derive(Clone, Copy, Default)]
struct Frame {
    phase: usize,
    start: Counts,
    children: Counts,
}
#[derive(Default)]
struct Profile {
    stack: [Frame; 8],
    depth: usize,
    exclusive: [Counts; 8],
    scopes: [usize; 8],
}
thread_local! { static PROFILE: RefCell<Profile> = RefCell::new(Profile::default()); }
fn event(phase: Phase, enter: bool) {
    let current = Counts::now();
    PROFILE.with(|p| {
        let mut p = p.borrow_mut();
        if enter {
            let depth = p.depth;
            assert!(depth < 8);
            p.stack[depth] = Frame {
                phase: phase as usize,
                start: current,
                children: Counts::default(),
            };
            p.depth += 1;
        } else {
            assert!(p.depth > 0);
            p.depth -= 1;
            let frame = p.stack[p.depth];
            assert_eq!(frame.phase, phase as usize);
            let total = current.minus(frame.start);
            p.exclusive[frame.phase].add(total.minus(frame.children));
            p.scopes[frame.phase] += 1;
            if p.depth > 0 {
                let parent = p.depth - 1;
                p.stack[parent].children.add(total);
            }
        }
    });
}
pub fn enable() {
    PROFILE.with(|p| *p.borrow_mut() = Profile::default());
    set_callback(event);
}
pub fn json() -> String {
    PROFILE.with(|p| {
        let p=p.borrow();assert_eq!(p.depth,0);
        let names=["equality_other","key","lookup","relevant_replay","exact_replay","capture","relevant_record","exact_record"];
        let rows=names.iter().enumerate().map(|(i,name)| {let c=p.exclusive[i];format!("{{\"phase\":\"{name}\",\"scopes\":{},\"allocation_calls\":{},\"requested_bytes\":{},\"deallocation_calls\":{}}}",p.scopes[i],c.calls,c.bytes,c.frees)}).collect::<Vec<_>>();
        format!("[{}]",rows.join(","))
    })
}
