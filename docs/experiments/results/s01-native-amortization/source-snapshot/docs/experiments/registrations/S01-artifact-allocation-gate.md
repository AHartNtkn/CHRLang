# Installed-plan allocation accounting gate

Add requested-allocation phase readings to the shared artifact runner using the
existing alloc-meter feature and allocator. Ordinary builds retain counter-free
ordinary timing; metered builds identify themselves explicitly and supply no primary
timing evidence. No engine or reference semantics change.

Measure source construction, preparation, setup, execution, observation, engine
and answer disposal, and prepared-state disposal separately. Capture readings before
formatting JSON. Exclude input/oracle construction and validation from phase traffic;
check exact live-byte restoration around each complete query including those inputs
and around the prepared lifetime. Initialize process stdout before the baseline.
Record phase start/end live bytes, peak live bytes and requested traffic. Peaks are
absolute process live requested bytes, not RSS or incremental phase ownership.

Build ordinary and metered release runtimes separately with no default features.
Compile generic, payload and subscription artifacts without LTO for this accounting
gate. For payload use all seven generic/native modes; for subscription use those
plus the three retained modes. Run sizes 0 and 16, four changing queries, twice per
cell in each build: 136 processes and 544 complete queries. Require independent
full answers, truthful completion, exact lifecycle time sums, correct allocator
labels, equal work-independent requested traffic on repeated metered runs, and
exact query/prepared live-byte restoration. Run the meter's allocation/resize
self-check in diagnostic processes. A live-byte discrepancy must be attributed and
repaired or explicitly accounted for before cost confirmation.

Each build/compile has a 60-second timeout and 4 GiB address-space cap; each process
has 60 seconds and 1 GiB, each query two million steps. One ordinary pair and one
metered pair establish compatibility/accounting, not performance estimates.
Cancellation and artifact-file lifetime remain separate endpoints. This gate gets
priority because unaccounted native/retained ownership would invalidate the pending
cost contrast; it does not authorize unrelated executor refinements.
