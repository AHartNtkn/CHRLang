# Checked readiness admits finite solving on the early-choice sources

Finite solving now accepts all 48 substantive queries whose readiness occurrence is present initially. The 48 late-readiness queries remain explicitly unsupported by this admission. The gate also found and repaired a common-source caller-adapter defect; all 264 previous common-source process observations replay exactly.

## What the new admission establishes

The admission check counts required kept occurrences whose predicates have no arguments. It requires enough distinct occurrences for every prefix rule and rejects a prefix that consumes any of them. Different rules can reuse the same kept occurrence, so their requirements are combined by maximum rather than addition.

These conditions explain why checking availability once is sufficient for this initial phase. Equality cannot change an argument-free occurrence. Prefix firings do not consume the required occurrences. Every normalized firing therefore still has the kept witnesses required by the source. The existing finite solver handles the remaining producer, matching, alias and resource restrictions.

The check applies only to initial admission. All subsequent caller work executes the original complete source, including its original kept-head conditions. It does not authorize later private work to bypass readiness, nor establish a general transformation for kept heads with arguments.

The 48 admitted sources combine early choices with equality chains, consumed edges, propagation receipts, nonmatching edges and failed siblings. Complete raw answers match the independently specified expectations. Repeated grouped queries reuse the prepared phase and full caller. Late-choice queries lack readiness initially and receive an explicit admission error, even though ordinary execution can later enable them and produce valid answers.

## The caller defect and its correction

A separate probe starts with `go(X)` and lets a later rule create `pick(X)`. The reference and Scan produce `a` and `b`. The common-source finite adapter instead left an unresolved `pick`, because it prepared the caller from only the suffix rules. Later work still needed the choice rule in the prefix.

The adapter now supplies the full original source to the existing caller bridge. The bridge, solver and reference implementation are unchanged. The finite bridge's existing future-work tests and lifecycle runner already use the full source; this correction aligns the common-source adapter with that responsibility.

Three complete probes now agree with both the reference and Scan: ordinary later private work, later private work with readiness, and later work after readiness has been consumed. In the last case the new `pick` correctly remains blocked. That result would be wrong if normalized initial rules were used unconditionally by the caller.

All previous common-source stdout and stderr observations replay exactly across 264 processes. Their earlier sources and binary are preserved and checked against the original hashes. This replay establishes the continued validity of those observed cases; it does not turn the failing future-work probe into a previously qualified case.

## Validation

| Evidence | Result |
|---|---|
| New semantic tests | Nine pass, covering readiness counts, aliases, duplicate alternatives, failure, interference, priority, future work and cancellation during admission |
| New and existing finite phase/bridge tests | 23 pass in each of default and `--no-default-features` builds |
| Prior common-source process observations | 264 exact replays |
| Substantive queries | 48 complete admissions; 48 explicit late-readiness exclusions |
| Changing-query prepared reuse | 48 complete observations across two rulesets |
| Caller counterexamples | Three agree independently with reference and Scan |
| Strict scoped Clippy and formatting | Pass |

Tests require enough occurrences for repeated kept heads while allowing two producer rules to reuse one readiness witness. They reject both ground and unknown-bearing kept heads with arguments, insufficient witnesses and prefix consumption of a required witness. Weighted duplicate alternatives and repeated query aliases preserve raw multiplicity. A pending finite-machine step can be cancelled, after which the prepared phase accepts another query and rechecks missing readiness.

This is correctness and eligibility evidence. No timings or allocation-efficiency conclusions are drawn. Preparation clones source structure and builds requirement counts; every query checks presence; the caller owns the full ruleset. Those costs must be included rather than credited as free analysis.

## Gate-boundary review: measure complete costs next

The selected admission hypothesis has a bounded positive result and concrete contrary cases. Further expansion to argument-bearing or late-enabled reads could admit more sources, but it is not needed to include a source-derived competitor on the early-choice cases now qualified. Those broader mechanisms remain open.

The next T078 investigation is equivalent lifecycle measurement for the qualified Rust paths and the native path. Include checked finite solving on admitted early-choice sources, retain explicit eligibility boundaries on late-choice sources, and keep the previously admitted prefix/finite source families as controls. Account for source construction/emission, analysis, preparation, changing queries, first/full observation, cancellation, retained consumers and disposal. Qualify counters, clock overhead and allocation scope before registering comparative repetitions.

A complete-cost study can now reveal whether domain solving, contextual execution or native reduction repays its added preparation and publication responsibilities. Native local claims/general terms and broader derivation or reuse mechanisms could change a larger architectural question, but need substantially more implementation before providing that contrast. Reassess them at the cost-runner qualification boundary, and sooner if a consequential obstruction appears. This priority selects the next evidence; it resolves none of those remaining directions.

## Evidence

[Registration](../registrations/S10-finite-kept-read.md), [checked phase](../../../research/chr-direct-conditional/examples/support/finite_kept_read.rs), [semantic tests](../../../research/chr-direct-conditional/tests/finite_kept_read_gate.rs), [current common-source adapter](../../../research/chr-direct-conditional/examples/native_common_source.rs), [failing caller probe](s10-finite-kept-read/late-private-probe.json), [corrected caller probes](s10-finite-kept-read/caller-probes.json), [prior observation replays](s10-finite-kept-read/common-replays.jsonl), [substantive observations](s10-finite-kept-read/substantive.jsonl), [prepared reuse](s10-finite-kept-read/reuse.jsonl), [audit](s10-finite-kept-read/audit.json), [historical hash validation](s10-finite-kept-read/historical-validation.json), [current hashes](s10-finite-kept-read/validation.json), [default tests](s10-finite-kept-read/tests-default.log), [no-default-feature tests](s10-finite-kept-read/tests-off.log) and [Clippy](s10-finite-kept-read/clippy.log) retain the evidence.
