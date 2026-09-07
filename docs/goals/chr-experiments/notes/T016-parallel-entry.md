# Independent E16 entry points

Read-only source inspection identifies two feasible parallel experiments that do
not depend on net services, native choices or the E15 operation broker. These are
independent experimental boundaries. The owned-equation protocol now passes its
[semantic gate](../../../experiments/results/E16-gate.md); certified regions remain
a separate implementation opportunity. No owner decision is needed to investigate
their costs under explicit experimental contracts.

## Owned equation workers with one source-state owner

The [persistent Machine](../../../../research/chr-persistent/src/continuations.rs)
already exposes pending_equation, complete_equation and ordinary step. The
[owned equation service](../../../../research/chr-reuse/src/lib.rs), Mode::Direct,
returns a finite-tree MGU/failure. Their [integration](../../../../research/chr-reuse/src/equation_search.rs)
has [E12 evidence](../../../experiments/results/E12-equations.md): 336 configurations
and replay, all 64 reference cases, and an alias/wakeup check.

Owned term/substitution fields are String, Vec, integers and BTreeMap, supporting
thread transfer; the implemented worker gate now compiles and exercises that boundary. Cursors contain
Rc state and remain on one owner. The mutable interned arena also remains there.
Changing Rc to Arc alone would not establish a concurrent source engine.

At a branch's next equation, export resolved operands and park its cursor under a
unique request ID. A worker computes only that equation. The owner installs the
complete result into that unchanged cursor and resumes source matching. Permit one
outstanding operation per branch; reject only that branch on unification failure.
Search exhaustion requires no runnable, parked or in-flight work. Only quiescent
source answers enter exact observation. This does not parallelize competing rule
firings or writes within one branch, and completion order may change answer prefixes.

The first question is whether operation concurrency across explicit-choice branches
repays projection, transport, installation and worker management. Keep five controls:
existing shared-arena scalar search; existing owned sequential service; new scheduler
with inline service; its identical one-worker transport path; and a worker-count sweep.
A win only over the owned control is insufficient if the competent arena scalar path
still wins. Retain E12's adverse representation result as contrary evidence.

Existing equation-repeat/unique/identity workloads form sequential chains. Add an
explicit-OR frontier with distinct answers, varying branch width and equation size.
Use equal-sized and skewed tasks, cheap identity, failure/occurs/alias/wakeup cases,
duplicate lineages, existing applications and a single-chain no-parallelism control.
Do not parallelize successive equations merely because their syntax looks disjoint.

Register task sizing/resource pilots, preparation and thread startup, warm-pool reuse,
projection/solve/install/observation costs, queue occupancy, payload/allocations and
peak/retained memory. Moving an owned payload through an in-process channel is not
wire serialization; distinguish projection copies from ownership transfer. Use
bounded admission and complete request accounting. Worker/protocol errors must not
be interpreted as semantic branch failure. Forced completion-order tests must preserve
full exhausted answers, raw multiplicity, aliases and source wakeups.

The existing Rust unifier and owner matching/observation are atomic. First investigate
bounded finite workloads without a bounded-service fairness claim. Resumable Rust
jobs, requeueing and owner service bounds are feasible further implementation work,
not external dependencies or closure grounds. Request protocol, worker lifecycle,
result handling and correctness gates are the concrete initial dependencies.

## Certified permanent regions

[Permanent factors](../../../../research/chr-factors/src/lib.rs) offer a coarser
independent boundary, with existing checks for aliases, multihead connectivity,
residual renaming, empty-region refutation and incremental products. Transfer owned
rules/query regions and construct each Rc engine inside its worker; keep product
assembly/global observation on the owner. Existing factored sequential execution and
persistent scalar search are the matched controls.

This requires exposing currently integrated factor/product scheduling, but does not
require equation workers or the broker first. It tests concurrency under certified
permanent independence; equation workers test available operation concurrency without
that restriction. Actual parallel branch engines, shared-store contention and competing
consumers are distinct further questions requiring explicit ownership/commit work.
Neither these entry points nor success on coarse factors closes those questions.

For later parallel rule commits, distinguish source freshness from the reference's
particular numeric allocation order. Worker-local or speculative allocation may need
a correspondence map rather than equal counters. Prove fresh-variable separation and
full continuation/observation correspondence; do not reject a correct architecture
solely because invisible handles differ. The initial equation-worker entry avoids
that complication by retaining source allocation and all commits on one owner.

## Selected first gate

The [E16 registration](../../../experiments/registrations/E16.md) selects bounded
lookahead over existing FIFO frontier entries, exporting only already-pending
equations and retaining cursors at their queue positions. Workers may finish out of
order, but the owner commits the front entry only. This preserves the existing
comparison schedule and exposes operation concurrency without a second source engine.
An inline controller, one-worker transport and two-worker transport use the same
request protocol. Completion-order scheduling remains a separate feasible follow-up.

Source review identified three boundaries that the gate makes explicit. Finish and
drain accepted requests before joining workers on a prefix stop; bounded channels
otherwise risk a shutdown deadlock. Perform exactly one bounded lookahead pass per
attempted source commit, without additional passes while receiving replies. Retain
reservations until commit, including completed buffered replies. These choices make
issued requests and aggregate speculative work deterministic under FIFO commits,
while completion order and buffered-result peaks can still vary.

Finally, pending_equation calls Arena::export, which increments the Machine's
dereference statistics. Extra speculative projection at a prefix is charged work,
not a changed source transition. Compare committed counts and observations directly,
and distinguish projection from source execution in the receipt. Focused tests must
establish actual simultaneous outstanding requests as well as correct results.

## Allocation and lifecycle measurement boundary

Read-only inspection of [the existing allocator](../../../../research/chr-reuse/examples/support/allocator.rs)
finds process-global AtomicU64 counters, not thread-local metering. Worker-private
EquationTable instances hold ordinary integer operation counters; collect and sum
those after joining. The existing heap counters cover worker allocations through
Rust's global allocator without a counter migration, but their global atomic updates
would themselves add contention to a parallel timing run. Compile the same future
timing harness with ordinary System allocation, and run allocation instrumentation
separately. Keep operation counting consistent across controls.

Take cold baselines before queues, workers, payload export and worker tables. Include
draining accepted work, joining and channel/table release in the cold total, with
separate readings before teardown and after release. Validate answers after those
readings. Warm-pool reuse needs an explicit quiescent phase boundary: allocator read
uses separate relaxed loads, and resetting peak while workers allocate can overwrite
a concurrent update. Do not infer a coherent phase snapshot from arbitrary live reads.

Requested bytes include each full new realloc size. The allocator's peak is a
high-water mark of hook accounting, not exact physical memory; realloc accounting
subtracts the old size before adding the new. OS thread stacks and allocations outside
the Rust global allocator are absent. Record stack configuration and process memory
separately before making whole-worker memory claims. Post-join live/requested totals
have a stronger boundary than an arbitrary concurrent sample. The current E12 probe
does not include teardown, so E16 needs this new lifecycle harness rather than merely
adding worker-count labels to its measurements.
