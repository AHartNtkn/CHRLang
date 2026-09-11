//! Allocation-free diagnostic scope notifications; absent from ordinary builds.
use std::cell::Cell;
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Phase {
    Equality,
    Key,
    Lookup,
    RelevantReplay,
    ExactReplay,
    Capture,
    RelevantRecord,
    ExactRecord,
    Admission,
    Constructor,
    ConstructorLookup,
    ValueCreate,
    Post,
    RowInsert,
    Columns,
    Incidence,
}
type Callback = fn(Phase, bool);
thread_local! { static CALLBACK: Cell<Option<Callback>> = const { Cell::new(None) }; }
pub fn set_callback(callback: Callback) {
    CALLBACK.set(Some(callback));
}
pub struct Scope(Phase);
impl Scope {
    pub fn new(phase: Phase) -> Self {
        CALLBACK.with(|c| {
            if let Some(f) = c.get() {
                f(phase, true);
            }
        });
        Self(phase)
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        CALLBACK.with(|c| {
            if let Some(f) = c.get() {
                f(self.0, false);
            }
        });
    }
}
