//! Single-threaded cumulative allocation attribution; no event allocates.
use crate::meter;
use chr_relational::deduction_profile::{Phase, set_callback};
use std::cell::RefCell;
#[derive(Clone, Copy, Default)]
struct Counts {
    calls: usize,
    bytes: usize,
    frees: usize,
    #[cfg(feature = "execution-profile")]
    ns: u128,
}
impl Counts {
    fn now() -> Self {
        let c = meter::checkpoint();
        Self {
            calls: c.allocation_calls,
            bytes: c.requested_bytes,
            frees: c.deallocation_calls,
            #[cfg(feature = "execution-profile")]
            ns: {
                static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
                START
                    .get_or_init(std::time::Instant::now)
                    .elapsed()
                    .as_nanos()
            },
        }
    }
    fn minus(self, b: Self) -> Self {
        Self {
            calls: self.calls - b.calls,
            bytes: self.bytes - b.bytes,
            frees: self.frees - b.frees,
            #[cfg(feature = "execution-profile")]
            ns: self.ns - b.ns,
        }
    }
    fn add(&mut self, b: Self) {
        self.calls += b.calls;
        self.bytes += b.bytes;
        self.frees += b.frees;
        #[cfg(feature = "execution-profile")]
        {
            self.ns += b.ns;
        }
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
    root: Option<Phase>,
    stack: [Frame; 8],
    depth: usize,
    exclusive: [Counts; 23],
    scopes: [usize; 23],
}
thread_local! { static PROFILE: RefCell<Profile> = RefCell::new(Profile::default()); }
fn event(phase: Phase, enter: bool) {
    let current = Counts::now();
    PROFILE.with(|p| {
        let mut p = p.borrow_mut();
        if p.depth == 0 && p.root.is_some_and(|root| root != phase) {
            return;
        }
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
        let names=["equality_other","key","lookup","relevant_replay","exact_replay","capture","relevant_record","exact_record","admission_other","constructor_other","constructor_lookup","value_create","post_other","row_insert","columns","incidence","execution_other","application_other","discovery","readiness","repair","cycles","observation"];
        let names = &names[..if cfg!(feature = "execution-profile") {23} else if cfg!(feature = "admission-profile") {16} else {8}];
        let rows=names.iter().enumerate().map(|(i,name)| {let c=p.exclusive[i];
            #[cfg(feature = "execution-profile")]
            let timing=format!(",\"diagnostic_ns\":{}",c.ns);
            #[cfg(not(feature = "execution-profile"))]
            let timing="";
            format!("{{\"phase\":\"{name}\",\"scopes\":{},\"allocation_calls\":{},\"requested_bytes\":{},\"deallocation_calls\":{}{timing}}}",p.scopes[i],c.calls,c.bytes,c.frees)}).collect::<Vec<_>>();
        format!("[{}]",rows.join(","))
    })
}

#[cfg(feature = "admission-profile")]
pub fn enable_admission() {
    enable();
    PROFILE.with(|p| p.borrow_mut().root = Some(Phase::Admission));
}

#[cfg(feature = "execution-profile")]
pub fn enable_execution() {
    enable();
    PROFILE.with(|p| p.borrow_mut().root = Some(Phase::Execution));
}
