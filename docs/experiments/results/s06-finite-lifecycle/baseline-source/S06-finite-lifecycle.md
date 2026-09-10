# Finite solver and resumed caller: bounded lifecycle pilot

T073 is active. The finite phase gate established a mechanism, not its costs.
This pilot asks whether eliminating private choice products repays source analysis,
owned symbolic states, interface transport and ordinary caller execution. The
strongest ready alternative is native graph/connected feasibility; complete costs
can now change the existing selective-source comparison at lower immediate effort.
Return to that priority comparison after this package; no architecture is assumed.

## Hypotheses and controls

H1: Finite solving lowers total requested allocation on larger selective cases
against explicit Scan, specialization and prepared-prefix execution. Conditional
execution is a competing way to avoid duplicated checking, not an assumed winner.
H2: Small cold and unselective sources can expose preparation, copying and caller
setup costs that erase the benefit. H3: Preserved duplicate weights avoid repeated
caller execution but cannot avoid materializing required raw answers. H4: Retained
answers and reusable artifacts account for their own live memory; query disposal,
consumer release, artifact release and prepared disposal restore the correct roots.

Six controls: finite phase plus reusable compiled caller; compiled Global Scan;
inferred specialization with Scan; prepared pure-prefix artifact with Scan;
conditional ascending/direct observer with Boolean identities and result cache;
conditional reverse/general observer with the same Boolean policy. Both
conditional builds enable inferred head dispatch, serial body accounting and
precise equality invalidation. No effect-contract restriction is enabled.

Common Cargo flags: `--release -p chr-direct-conditional --no-default-features
--features experiment,head-dispatch,serial-body-accounting,equality-invalidation,
support-identities,support-result-cache --example finite_lifecycle` (one
comma-separated feature argument). Reverse adds `support-reverse-order`.
Diagnostics add `alloc-meter`. Compile-time guards reject engine/kernel/observer
work counters. Ordinary timing uses the ordinary allocator, separate binaries and
processes. The meter measures requested heap bytes, not RSS.

## Sources, exact cells and repetitions

Families: `oldest` and `newest` use the established opposite-arrival selective
source; `all` lets both atom cases pass and retains a full choice-list witness;
`duplicates` adds another successful `t` alternative. All include the ordinary
consuming token boundary, both output aliases and changing query tags. Payload
size is 4. Reused queries have depths n through n+reuse-1 and alternate tag/order.
The solver extracts domains from the actual source; expected answers are used only
for validation. Its checked private prefix has six rules.

For every family and control, run these eight cells (depth, reuse, keep, cancel,
final failure, work):

1. (0, 1, 0, none, false, 0)
2. (4, 1, 0, none, false, 2)
3. (4, 4, 0, none, false, 2)
4. (4, 4, 4, none, false, 2)
5. (4, 4, all, none, false, 2)
6. (4, 4, 0, none, true, 2)
7. (4, 4, all, step4, false, 2)
8. (8, 1, 0, none, false, 2)

For `all` and `duplicates`, additionally run (4, 4, all, answer2, false, 2).
Cancellation affects query zero only; subsequent queries complete. `step4` stops
after four service calls; it is not equal useful work across implementations.
`answer2` stops after two raw answers and exercises partly consumed solution or
repetition state. These give 204 cells.

Run two isolated allocation processes per cell, shuffled with seed 7940: 408 runs.
Then run five ordinary-allocator processes per cell, shuffled with seed 7941:
1,020 runs. Freeze the exact runner, sources, registrations, dependency sources and
binaries before the matrix. Every process has a 1 GiB address-space limit, 60 CPU
seconds, 75 wall seconds and a 2,000,000 combined service-call bound per query.
The finite solver's previously recorded admission limits remain in force. A
cutoff or error halts the matrix for investigation; it is not architectural loss.

## Measurement boundaries and ownership

Record source construction and reusable preparation, each query's input, setup,
private solving, transport, caller setup, caller execution/observation and their
disposal. Record consumer release, artifact release and prepared disposal. Ordinary
controls use their actual setup/execution/disposal path. Prepared-prefix artifacts
are cached by ordered predicate/arity signature; first construction is charged to
setup, and final artifact release is charged separately.

One ordinary equality-replay rule is prepared once for the finite caller. It uses
a source-fresh predicate, rejects a query collision and omits reflexive equations.
Weighted caller answers are materialized individually before consumer delivery.
First-answer latency starts at query input construction and overlaps the phase
sum; never add it to total cost. Answer export is included in execution/observation,
not reported as a separately isolated kernel time.

Fixed-capacity result/phase and consumer-container buffers are allocated outside
measurement in every control. Their capacity is chosen for the maximum registered
answers and rows, independent of retention policy; dynamic answer payloads remain
charged. No JSON or oracle work is inside measured phases. Before measurement,
every exact mode/query completes and its full answer multiset is compared with
independent scalar semantics and the analytical source result. Actual measurement
checks endpoints/counts; the complete replay is separate and warms within-process
execution. This is not a cold-process startup measurement.

Require exact repeated allocation records, phase live-byte continuity, complete
owner restoration, consumer-independent allocation/work where applicable, and
matching endpoints/work counts between diagnostic and ordinary builds. Preserve
all phases rather than infer total cost from solver execution alone.

## Interpretation and follow-up

Compare requested totals and peak growth pointwise. For timings report five-sample
medians and ranges only; these are exploratory measurements, not significance or
speed-adoption claims. A median difference of at least 20% flags a potentially
consequential attribution, not a universal practical threshold or workload weight.
Investigate any material loser whose cost is dominated by an avoidable boundary or
implementation defect. Challenge gains with the registered adverse families,
retention, cancellation and cold/reused preparation.

Existing smoke runs qualified ownership and revealed reflexive transport work,
which is addressed before this matrix. They are exploratory sizing, not paired
confirmation. If a frozen implementation needs a consequential repair, preserve
its runs and register the new comparison before timing it.

Host compilation, user-program native compilation, process startup and independent
validation are excluded. No complete architectural lifecycle superiority, mandatory
language restriction or universal winner follows from this pilot.
