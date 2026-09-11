use crate::meter;
use chr_direct_choice::demand::candidate_profile::{Phase, set_callback};
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
    fn minus(self, old: Self) -> Self {
        Self {
            calls: self.calls - old.calls,
            bytes: self.bytes - old.bytes,
            frees: self.frees - old.frees,
        }
    }
}
#[derive(Default)]
struct Profile {
    active: Option<(usize, Counts)>,
    totals: [Counts; 3],
    scopes: [usize; 3],
}
thread_local! {static PROFILE:RefCell<Profile> = RefCell::new(Profile::default());}
fn event(phase: Phase, enter: bool) {
    let now = Counts::now();
    PROFILE.with(|p| {
        let mut p = p.borrow_mut();
        let index = phase as usize;
        if enter {
            assert!(p.active.is_none());
            p.active = Some((index, now));
        } else {
            let (opened, start) = p.active.take().unwrap();
            assert_eq!(opened, index);
            let delta = now.minus(start);
            p.totals[index].calls += delta.calls;
            p.totals[index].bytes += delta.bytes;
            p.totals[index].frees += delta.frees;
            p.scopes[index] += 1;
        }
    });
}
pub fn enable() {
    PROFILE.with(|p| *p.borrow_mut() = Profile::default());
    set_callback(event);
}
pub fn json() -> String {
    PROFILE.with(|p| {let p=p.borrow();assert!(p.active.is_none());let names=["arguments","environment","selection"];
    let rows=names.iter().enumerate().map(|(i,name)| {let c=p.totals[i];format!("{{\"phase\":\"{name}\",\"scopes\":{},\"allocation_calls\":{},\"requested_bytes\":{},\"deallocation_calls\":{}}}",p.scopes[i],c.calls,c.bytes,c.frees)}).collect::<Vec<_>>();
    format!("{{\"event\":\"candidate-profile\",\"rows\":[{}]}}",rows.join(","))
})
}
