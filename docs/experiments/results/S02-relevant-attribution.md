# Relevant-state keys dominate one allocation loss; demand lifecycle is next

Building relevant-state keys accounts for most execution allocation in the changed-context witness. A cheaper key could reverse some comparisons with recomputation or scanning. It cannot, by itself, reverse the allocation comparison with source elimination in any of the 32 inspected scenarios.

Proceed to the demand dependency lifecycle comparison under T071. Keep sound cheaper recognition and relevant-deduction timing required under T072. This is an allocation attribution and a prioritization decision; it establishes no speed ranking or general rejection of integrated execution.

## What the experiment measures

The candidate reuses equality deductions when the parts of the caller's state reachable from the two equality inputs agree with a recorded deduction. It constructs an owned key by traversing that reachable structure and recording identities, representatives and constructor descriptions. Reusing the deduction still updates the current caller's maps; it does not adopt another caller's complete state.

The five compared modes are uncached contextual equality, exact-state shared deductions, relevant-state shared deductions, and persistent-map versions of both caches. The matrix uses single/shared/distinct/changed families, depths 0/16, one/four changing queries, a resource token, and immediate/all answer retention. There are 32 scenarios, five modes and two diagnostic repetitions: 320 processes. This is the token-present subset of the [ownership experiment](S02-relevant-ownership.md), not a new timing matrix.

Callbacks record cumulative allocation counters around the equality step and its nested operations. Each category reports exclusive traffic: child scopes are subtracted from their parent. Equality work outside the named nested operations forms the “other equality” category. Requested allocation is neither resident memory nor a measure of execution time.

## The consequential changed-context case

The witness uses depth 16, four changing queries, a resource token and retained-all answers. All quantities below are requested bytes. Session totals include the measured preparation, query and disposal phases from the ownership comparison.

| Execution path | Session total | Execution and observation | Key construction | Deduction replay |
|---|---:|---:|---:|---:|
| Contextual recomputation | 669,196 | 619,332 | 0 | 0 |
| Exact-state cache | 1,311,396 | 1,261,532 | 0 | 0 |
| Relevant-state cache | 1,292,590 | 1,242,726 | 941,208 | 42,264 |
| Persistent exact-state cache | 1,091,764 | 1,041,900 | 0 | 0 |
| Persistent relevant-state cache | 1,667,574 | 1,617,710 | 941,208 | 329,574 |

The relevant caches each construct 296 keys and replay 210 deductions. The changed contexts produce no exact-state replay in this witness. Relevant reuse therefore exercises its intended benefit; this is not a loss from failing to reuse anything.

The relevant cache's key construction requests 75.7% of its execution/observation bytes. Its session excess over recomputation is 623,394 bytes. Holding every other allocation fixed, saving more than 66.2% of key bytes would reverse that particular session comparison. This is a sensitivity calculation, not a claim that such a saving is implementable or sufficient for a runtime gain.

Persistent replay has an additional cost. With the same 210 replay scopes, it requests 329,574 bytes versus 42,264 for ordered-map replay. Source inspection places caller-local parent/descriptor updates and queued child equations inside this scope. The measurement identifies their combined traffic; it does not separate each map operation or establish that the difference is intrinsic to persistence.

The remaining exclusive categories make the accounting checkable:

| Category | Contextual | Exact cache | Relevant cache | Persistent exact | Persistent relevant |
|---|---:|---:|---:|---:|---:|
| Other equality | 531,688 | 1,126,304 | 146,674 | 906,672 | 234,348 |
| Lookup | 0 | 0 | 0 | 0 | 0 |
| Capture | 0 | 4,224 | 9,656 | 4,224 | 9,656 |
| Relevant recording | 0 | 0 | 15,280 | 0 | 15,280 |
| Exact recording | 0 | 43,360 | 0 | 43,360 | 0 |
| Outside equality | 87,644 | 87,644 | 87,644 | 87,644 | 87,644 |

Add these rows to key construction and replay to recover execution/observation allocation. Lookup's zero allocation does not imply zero CPU cost. “Other equality” includes allocations outside the named scopes; scope-local deallocations can include key destruction without changing where its original allocation is attributed.

## How much could key-only repair change?

Subtract all measured key-construction bytes from each session while leaving all other bytes fixed. This deliberately optimistic calculation asks whether key-only allocation work could even reverse the inspected contrast. It does not predict the allocations of a future implementation, whose validity checks, retained data and replay could differ.

| Candidate | Control | Currently lower allocation | Lower under zero-key calculation |
|---|---|---:|---:|
| Relevant | Contextual recomputation | 0/32 | 16/32 |
| Relevant | Exact-state cache | 6/32 | 24/32 |
| Relevant | Persistent exact-state cache | 0/32 | 20/32 |
| Relevant | Scan | 10/32 | 14/32 |
| Relevant | Source elimination | 0/32 | 0/32 |
| Persistent relevant | Contextual recomputation | 2/32 | 6/32 |
| Persistent relevant | Exact-state cache | 0/32 | 24/32 |
| Persistent relevant | Persistent exact-state cache | 0/32 | 8/32 |
| Persistent relevant | Scan | 12/32 | 14/32 |
| Persistent relevant | Source elimination | 0/32 | 0/32 |

These are counts of experimental scenarios, not application frequencies or workload weights. Source elimination is an applicable strong control for this family; its narrower eligibility prevents this result from selecting it for arbitrary consuming/equality programs. The surviving possible reversals against recomputation and scanning justify retaining a key-repair experiment.

## Why the attribution is credible

An independent receipt audit checks all 544 frozen source hashes, the original binary hash, 320 complete raw results, both attribution repetitions and the exact corresponding ownership records. Allocation/deallocation counts and requested bytes agree exactly. Live and peak values agree after subtracting each process's premeasurement baseline, which includes executable argument storage. The outside-equality byte residual also agrees across all five modes in every one of the 32 scenarios.

A separate single-thread program exercises nested scopes with known allocations: 64 bytes in the outer equality scope, 128 and 512 in two key scopes, and 256 in a nested replay scope. The profiler reports 64/640/256 exclusive bytes, four allocations and four frees. The meter retains the expected 576-byte peak above baseline and restores live bytes. This directly checks subtraction, repeated phases and preservation of peak tracking without relying on the source workload's aggregate results.

Current counter-free and profile-enabled builds each pass four relevant-deduction source tests and two lifecycle harness tests. Those tests include independent complete answers, invalid relevant-state reuse, source resource distinctions and interruption/ownership behavior. Strict scoped Clippy passes. The production source changes only add feature-gated scope notifications; the callback module and calls are absent when `deduction-profile` is disabled. The feature forces the allocation meter and is not a primary timing configuration.

Source inspection finds the instrumented `Store::step` call in the engine's execution path. Preparation and independent answer validation do not call it in this harness. Callback storage is fixed-size thread-local state initialized before measurement; no event constructs heap data. JSON reporting follows all measured disposal. The original registration uses 60-second wall/CPU, 1-GiB address-space and 2,000,000-turn limits, with no reported cutoff.

## Decision and required follow-through

This finishes package two after the relevant-deduction breadth review. The preceding package established ownership; this one identifies a consequential allocation cause. Neither supplies counter-free timing, sustainable retention, or a cost result for distinct flat/CHR-expressed/local-rewrite organizations.

T071 demand lifecycle is the next selected investigation. It already has independent source gates and a miss-reuse control that removes millions of force entries in one case while adding lookups in others. Measuring whether that different executor's saved work pays is more valuable now than immediately refining the equality key. T076 reusable symbolic formulas and union remain the strongest distinct representation alternative and follow the demand comparison under the [current sequence](../next-cycle.md).

T072 retains a concrete repair question: can a recorded relevant read set be validated without reconstructing its full owned key on every attempted reuse? Any candidate must preserve changed-descendant, constructor-cycle, caller-binding and resource-identity checks and face low-reuse overhead. This report does not select that implementation. Reconsider the repair at the demand lifecycle result or a concrete obstruction; any intervening correctness, attribution or qualification package counts toward the four-package breadth review.

A key-only improvement cannot overturn the inspected source-elimination allocation contrast, but it could change other contrasts and more general source eligibility. That is why the repair remains required and why the present result is not an architectural rejection. The research goal remains active.

## Evidence and reproduction

- [Prospective registration](../registrations/S02-relevant-attribution.md), [original driver](../../../research/chr-relational/experiments/relevant_attribution.py), [frozen sources and binary](s02-relevant-attribution/freeze.json), [160 cell summaries](s02-relevant-attribution/results.json) and [320 raw logs](s02-relevant-attribution/runs/).
- [Independent auditor](../../../research/chr-relational/experiments/relevant_attribution_audit.py) and [audit result](s02-relevant-attribution/review-audit.json). Run `python research/chr-relational/experiments/relevant_attribution_audit.py` from the checkout to verify the archived comparison without rerunning timings.
- [Known-allocation program](s02-relevant-attribution/attribution-self-check.rs), [its output](s02-relevant-attribution/attribution-self-check.log), [counter-free tests](s02-relevant-attribution/review-counter-free.log), [profile tests](s02-relevant-attribution/review-profile-tests.log), [Clippy](s02-relevant-attribution/review-clippy.log) and [validation commands](s02-relevant-attribution/review-validation.json).

The original driver refuses to overwrite its run directory. All comparative results remain tied to the frozen implementation. The independent audit and the known-allocation check add validation; they do not replace the original measurements.
