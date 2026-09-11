//! Optional allocation-free attribution of positive context inclusion.
use std::cell::{Cell, RefCell};
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Site {
    Birth,
    Consumed,
    Recursive,
    Result,
    ChoiceBirth,
    Pattern,
    Lift,
    Finite,
    AnswerObligation,
    AnswerResidual,
    AnswerResult,
    Tick,
}
pub const NAMES: [&str; 12] = [
    "birth",
    "consumed",
    "recursive",
    "result",
    "choice_birth",
    "pattern",
    "lift",
    "finite",
    "answer_obligation",
    "answer_residual",
    "answer_result",
    "tick",
];
#[derive(Clone, Copy, Default, Debug)]
pub struct Stats {
    pub calls: usize,
    pub accepted: usize,
    pub support: usize,
    pub context: usize,
    pub visited: usize,
    pub steps: usize,
    pub seeks: usize,
}
type Callback = fn(Site, bool);
thread_local! {
 static TOTALS: RefCell<[Stats; 12]> = RefCell::new([Stats::default(); 12]);
 static ACTIVE: Cell<Option<Site>> = const { Cell::new(None) };
 static CALLBACK: Cell<Option<Callback>> = const { Cell::new(None) };
}
pub fn reset() {
    assert!(ACTIVE.get().is_none());
    TOTALS.with(|p| *p.borrow_mut() = [Stats::default(); 12]);
}
pub fn snapshot() -> [Stats; 12] {
    TOTALS.with(|p| *p.borrow())
}
pub fn set_callback(f: Callback) {
    CALLBACK.set(Some(f));
}
pub(crate) fn work(visited: usize, steps: usize, seeks: usize) {
    if let Some(site) = ACTIVE.get() {
        TOTALS.with(|p| {
            let mut p = p.borrow_mut();
            let r = &mut p[site as usize];
            r.visited += visited;
            r.steps += steps;
            r.seeks += seeks;
        });
    }
}
pub(crate) fn measure(
    site: Site,
    support: usize,
    context: usize,
    f: impl FnOnce() -> bool,
) -> bool {
    assert!(ACTIVE.replace(Some(site)).is_none());
    if let Some(c) = CALLBACK.get() {
        c(site, true);
    }
    let result = f();
    if let Some(c) = CALLBACK.get() {
        c(site, false);
    }
    ACTIVE.set(None);
    TOTALS.with(|p| {
        let mut p = p.borrow_mut();
        let r = &mut p[site as usize];
        r.calls += 1;
        r.accepted += usize::from(result);
        r.support += support;
        r.context += context;
    });
    result
}
