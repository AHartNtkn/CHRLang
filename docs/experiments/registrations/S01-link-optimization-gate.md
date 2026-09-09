# Cross-crate optimization gate for T070

Before runs, compare ordinary optimized linking with ThinLTO on the same fixed
bitcode-bearing runtime. This is a correctness, compiler feasibility and static
code inspection gate, with exploratory compilation costs; no runtime ranking.
It answers whether a stronger available configuration can optimize the remaining
runtime boundaries without breaking independent ruleset compilation.

Build once in a new dedicated target with RUSTFLAGS=-Cembed-bitcode=yes, offline
release, chr-compiled experiment feature and no default features. Record whether
the target existed before construction. Compile the same five previously validated
sources (generic, chain, payload, subscription, dispatch16) with edition 2024,
opt-level 3 and one codegen unit, once with lto=off and once with lto=thin. Validate
source hashes against the preceding artifact gate before compiling. Runtime rlib
hashes must remain fixed throughout. Record complete commands, toolchain, per-build
wall time, child CPU, executable sizes and hashes. One repetition is sufficient
for feasibility; these costs are exploratory observations, not stable estimates.

Both configurations run all seven applicable execution modes for four families,
base sizes 0 and 5, three changed queries each: 112 processes and 336 complete queries.
Use the same independent scalar answers and phase-total checks as the artifact
gate. Each configuration must reject a payload artifact paired with chain rules
before publishing output. Inspect generated chain continuation bodies and resolve
ELF relative relocations to symbols. Remaining static call sites are not dynamic
call counts or a performance measurement. Missing symbols alone do not prove
inlining; report inspectable bodies and resolved targets explicitly.

Each build, compile or run has a 60-second timeout; compilation has a 4 GiB address
space ceiling and runtime 1 GiB, with 2 million source steps per query. Abort and
investigate any failure without replacing its receipt. Runs have ordinary allocation
and no engine/kernel/observer counters. No source or engine behavior changes are
part of this comparison.

If ThinLTO works, retain both configurations as candidates for cost sizing; do not
select it solely for smaller code or fewer calls. If unavailable, identify the
concrete toolchain/build boundary and repair it when feasible. If neither removes
the relevant calls, inspect the surviving abstraction before attributing native
costs to architectural obligations. After this bounded gate, reconsider lifecycle
sizing against direct pull-tabbing/derivation reuse. This gate has priority because
an avoidably weak compilation control could invalidate the imminent T070 comparison.
