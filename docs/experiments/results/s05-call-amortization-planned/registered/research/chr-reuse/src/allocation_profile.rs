//! Optional requested-allocation attribution. Nested scopes do not provide peaks.
#[cfg(feature = "stage-alloc")]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    pub calls: [u64; 5],
    pub bytes: [usize; 5],
    pub allocations: [usize; 5],
}
#[cfg(feature = "stage-alloc")]
impl Profile {
    pub(crate) fn record(
        &mut self,
        stage: usize,
        reading: chr_compiled::experiment::meter::Reading,
    ) {
        self.calls[stage] += 1;
        self.bytes[stage] += reading.requested_bytes;
        self.allocations[stage] += reading.allocation_calls;
    }
}
macro_rules! measure {
    ($owner:expr, $stage:expr, $body:expr) => {{
        #[cfg(feature = "stage-alloc")]
        let start = chr_compiled::experiment::meter::begin();
        let result = $body;
        #[cfg(feature = "stage-alloc")]
        $owner
            .profile
            .record($stage, chr_compiled::experiment::meter::end(start));
        result
    }};
}
pub(crate) use measure;
