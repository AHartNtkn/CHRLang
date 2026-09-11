# Selective probing preserves source selection and avoids candidate work

**The prepared probe avoids discovery without reordering committed matches.** In the larger right-bound source, Global candidate visits fall from 515 to seven; Active visits fall from 905 to 651. Complete answers and original occurrence tuples are preserved, including ambiguous consuming matches. This qualifies an implementation for lifecycle investigation, not an architecture winner.

## What the implementation does

Preparation records later heads that share logical slots with each current head. When the current pool has no useful key and a later head has a smaller keyed bucket, the probe matches that later bucket on private frames. It uses the resulting keys to collect possible current occurrences, unions their IDs in ascending order, then lets ordinary source-order matching, guards and commitment proceed.

The probe neither commits a later occurrence nor changes head identities. Its candidate set may contain unnecessary entries: for example, it does not use distinct-occurrence or guard checks to eliminate a probe early. This is intentional overapproximation. Every viable source tuple must retain its current occurrence; ordinary selection determines which tuple actually commits. Unknown projected keys admit the complete current pool rather than excluding open matches. Source effects remain outside probing.

This code is isolated behind `selective-probe`; the default remains the existing implementation. It uses existing indexes and adds prepared links, temporary frames and an ID set. Those allocations and preparation costs must be measured before a total-efficiency claim.

## Measured work

These are complete-session diagnostic counts for the size-128 right-bound, left-first, successful last-key source with forward insertion and Indexed access.

| Responsibility | Global control | Global probe | Active control | Active probe |
|---|---:|---:|---:|---:|
| Ordinary candidate visits | 515 | 7 | 905 | 651 |
| Structural tests, including probes | 1,026 | 14 | 1,677 | 1,171 |
| Index lookups | 768 | 14 | 1,029 | 781 |
| Additional later-occurrence probes | 0 | 2 | 0 | 1 |

These counters have different units and are not summed into a synthetic runtime. Probe visits are additional inspected occurrences, not hidden inside the smaller ordinary candidate count.

Across both 384-configuration source families, all 384 Scan configurations retain identical work. Among the 384 Indexed configurations, ordinary candidate visits fall in 120 and remain unchanged in 264; none increase. The bound-request family accounts for 96 reductions; the initial control supplies 24. Available keys and activation still determine when the plan is useful.

## Two observed costs were repaired

The low-selectivity diagnostic initially inspected 256 later occurrences in two probes at width 128. The keyed bucket was as broad as the current pool. The implementation now checks maintained bucket cardinalities and does not probe that case. Its final candidate and structural counts match the control; index lookups are 389 versus 387, charging the two eligibility decisions.

The first frozen qualification also exposed duplicate current-key selection. A neutral Indexed/Global case used 1,536 index lookups instead of the control's 1,024, without starting a probe. Pool materialization now accepts the key already selected. A separately registered, freshly frozen confirmation reproduces every default work row exactly and removes that duplicated search. Genuine eligibility work remains counted: 48 bound-source configurations add index lookups without saving candidate visits.

Neither repair excludes the adverse fixture or weakens its expected answer. The earlier diagnostics and both frozen confirmations are retained.

## Correctness evidence

Each frozen campaign contains 18 bounded processes spanning feature on/off and metrics on/off. Both partner-order families contribute 4,608 complete sessions and 48 ambiguous-match sentinel executions. Another 12,288 small ground sessions independently compute the lexicographically first legal occurrence tuple over every pair of four-bit relation masks, both head orders and an optional guard. Exact traces and complete residual multisets must match that independent result.

There are also 192 alias/constructor/duplicate/binding-update comparisons against Scan, checking both complete answers and exact traces, plus 36 broad-pool sessions. The nonground comparisons are differential evidence; the ground tuple oracle is independent. All metrics-on work rows repeat exactly. Metrics-off runs preserve the same source results. The no-op probe fails the initial work test at 515 candidate visits; the implemented probe passes it.

Existing access, generated-access and join-screen regression tests pass, including late binding and fresh replacement. Scoped Clippy passes. The reference interpreter is unchanged. This does not claim generated-plan speed: the new plan has not yet been compared as prepared data versus native generated code.

## Next decision

T082 next qualifies ownership and service cost: prepared link storage, temporary frames and ID sets, broad/duplicate projections, changing queries, cancellation and the maximum work within one engine step. Probe construction currently occurs inside pool creation, so source-step counts alone cannot establish bounded service latency. Measure that responsibility directly and make it resumable if the work obstructs the required service contract.

Then register ordinary-allocator timing with preparation, query setup, execution, observation and disposal, using separate allocation diagnostics and the same semantic controls. Compare the complete costs against the current indexed implementation and the legal source-order opportunity. Do not infer an architecture choice from the candidate reduction.

This is package two after the [portfolio review](S08-context-portfolio-review.md). Lifecycle qualification has immediate value because there is now a source-preserving mechanism with measured saved work. At that gate compare continued planning with fresh memo attribution, integrated admission and direct solving. Those broader investigations and the full goal remain active obligations.

Registrations: [probe gate](../registrations/S01-selective-probe.md), [key-reuse repair](../registrations/S01-probe-key-reuse.md). Frozen analyses: [first qualification](s01-selective-probe/analysis.json), [repaired qualification](s01-probe-key-reuse/analysis.json). Run `python3 research/chr-compiled/experiments/audit_selective_probe.py` to verify both archives, repetitions, case coverage and matched work. Source: [probe implementation](../../../research/chr-compiled/src/selective_probe.rs).
