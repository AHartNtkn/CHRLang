# R04 matched cold/reuse lifecycle pilot

Prospective T032 pilot, registered before comparative runs. Finite native/Boolean and compiled source-search gates pass. Test whether finite inference/elimination of source execution earns its preparation, enumeration and lifetime costs. A direct conditional resource protocol is the strongest alternative; it has more simultaneous semantic uncertainties, whereas this complete-path contrast is now bounded and independently checkable. Do not select a universal winner or workload distribution.

## Predictions and controls

Native AC and support CNF can propagate relationships before explicit source assignments; arc-consistent contradictions require search on both. Conflict CNF is an inference-strength control, not the only Boolean candidate. Weak relations expose enumeration/output and preparation costs with little propagation to save. Z3 may benefit from learned information across query scopes, but startup, models and blocking can outweigh that benefit. Native trails may restore cheaply; compiled copied frontiers pay for source resources and access structures. These are hypotheses, not measured directions.

Six variants: native, support, conflict, generated-global-scan, generic-global-scan, generated-active-indexed. Compiled rule order is given, forbid, choose (finite fixture order3), offering early supplied-binding and failure service under global selection. Initial forbidden occurrences precede chooser occurrences; query additions are givens. Active reuse appends given activations behind the existing queue and may branch before supplied bindings; its result concerns that joint schedule, not an isolated indexing effect. The global control is the given-first comparator. All variables remain explicit source choosers, and full outputs expose every assignment. Native/Boolean normalization retains residual duplicates. Different complete search heuristics are part of the respective algorithms; initial support/AC inference was separately matched.

## Exact registry

Families are in `research/chr-finite/experiments/cost.rs`:

- weak: no forbidden pairs, n=4,6;
- chain: adjacent variables must be equal, n=4,8;
- unsupported: variable0 value0 has no support at variable1, others unconstrained, n=4,6;
- contradiction: a four-variable all-different clique over three values, remaining variables unconstrained, n=4,8;
- asymmetric: each adjacent edge forbids (a,a+1 mod3) and (0,0), n=4,8.

Ten workload cells ×six variants ×two lifecycles =120 cells. Cold has one preparation and one unconstrained-givens query. Reuse has one preparation and four queries: no givens; variable0=0; last variable=1; no givens again. This is actual changed-query reuse, not a repetition multiplier. Five timing processes plus one process-memory run per cell =720 processes,1,800 complete query lifecycles requested. Seed20260915 shuffles all jobs; save order, no warmup exclusion or adaptive repetition. Every query is independently checked by exhaustive assignments and full residual reconstruction outside measured intervals. Cutoffs preserve sound prefix checks without claiming completeness.

## Matched input and lifetime boundary

Each backend receives the same fixed topology and changed givens. Native preparation builds the normalized relation and immutable native support indexes. Boolean preparation builds its relation/CNF/Z3 objects; native indexes are not constructed for it. Cold compiled execution consumes one ordinary branch directly, without a reusable-template copy. Reused compiled preparation builds rules plus an initial query template with posted stores/indexes/activation but no source application or choice. Query start clones that state and posts givens using its retained source-variable scope. Fresh-query equivalence and branch independence are independently checked. No assignments are supplied outside source execution.

Time preparation, query setup, execution between delivered results, actual reconstruction, result-state disposal, query disposal, answer disposal and prepared disposal. Query request wall time includes all glue and collection through query disposal. All backends retain the complete Answer batch until outside-interval validation; subsequent batch disposal is separately charged. Four-query lifecycle sums preparation/disposal and request+answer-disposal intervals. Full output retention is an explicit common observation policy, not a streaming memory claim. Native initial AC is in setup, Boolean initial solving is in execution: complete first-answer and lifecycle are the matched endpoints. Do not rank execution phase alone.

Fixture/oracle construction is outside primary timing and reported as harness time. Native source Rust compilation, bundled generated code compilation, dynamic-library loading and process startup are not isolated per ruleset; no full architectural compilation amortization claim. Process-level elapsed time is separate from the native lifecycle endpoint.

## Build, resource and interpretation controls

Build installed Cargo release defaults with `cargo build --release -p chr-finite --no-default-features --features experiment --bin chr-finite-cost --target-dir /tmp/chr-r04-lifecycle`. Freeze measured inputs, runner/registration, generated source, resolved features, compiler/library versions and binary hashes before comparative execution. Assert actual compiled engine and kernel counters disabled in every process. Ordinary allocator; no Rust allocation-meter claim. Clock calibration required, no fixed overhead subtraction.

Run serially pinned to minimum permitted CPU affinity, no concurrent builds/experiments. Per child:2GiB address space,20 CPU seconds,30 wall seconds, no core dumps. Batch bound15minutes. Compiled query budget1,000,000 service ticks; native/Boolean finite domain at most8 variables. Preserve timeouts/cutoffs and continue other cells; stop on validation or instrumentation failure. UNKNOWN is an error, not UNSAT. Unsuccessful cells do not receive completed-lifecycle medians.

Memory runs use the same counter-free binary with phase-boundary `/proc/self/status` RSS reads and GNU `/usr/bin/time` maximum resident set. Record before preparation, after preparation, retained-answer and post-answer query boundaries, and after prepared disposal. Whole-process RSS includes code, allocator/library retention, fixtures/oracle and output batches. It covers Z3 C allocations but is not an isolated live-heap measurement; no arbitrary baseline subtraction. Memory-run timings are not primary results. Bounded repeated-query RSS is evidence about this batch only, not a leak proof.

Report per-cell five-process medians/ranges, first complete observation (absent for UNSAT), all phases, all resource failures and memory. Directions below10% or overlapping ranges are inconclusive; phase directions below100× median clock calibration are inconclusive. Do not aggregate workloads. A large solver/native benefit needs matching preparation/reuse and output cost; an adverse result concerns this encoding/runtime/observation policy. Further tuning requires a specific architecture decision-changing uncertainty. Costs of source compilation, conditional activation, general language contracts and parallel execution remain open.
