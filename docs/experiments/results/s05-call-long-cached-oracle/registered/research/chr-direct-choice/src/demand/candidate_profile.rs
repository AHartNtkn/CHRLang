//! Allocation-free notifications for three nonnested resource-matching operations.
use std::cell::Cell;
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Phase {
    Arguments,
    Environment,
    Selection,
}
type Callback = fn(Phase, bool);
thread_local! {static CALLBACK: Cell<Option<Callback>> = const {Cell::new(None)};}
pub fn set_callback(callback: Callback) {
    CALLBACK.set(Some(callback));
}
pub struct Scope(Phase);
impl Scope {
    pub fn new(phase: Phase) -> Self {
        CALLBACK.with(|c| {
            if let Some(callback) = c.get() {
                callback(phase, true);
            }
        });
        Self(phase)
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        CALLBACK.with(|c| {
            if let Some(callback) = c.get() {
                callback(self.0, false);
            }
        });
    }
}
