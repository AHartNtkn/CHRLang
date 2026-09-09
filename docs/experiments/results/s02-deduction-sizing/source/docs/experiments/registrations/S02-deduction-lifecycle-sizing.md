# Shared equality deductions: lifecycle sizing

The [mechanism gate](../results/S02-shared-deduction-gate.md) establishes exact equality-transition reuse with local source effects. This bounded sizing package tests whether its saved merge work may repay lookup, retained maps and copy-on-write updates. It precedes T073's reusable lowering because the new mechanism has no cost evidence; revisit selection after attribution/confirmation rather than refining it indefinitely.

## Source families and controls

All sources unify constructor chains ending in an unknown with corresponding chains ending in `a`. `single` has one path; `shared` has four source alternatives using the same input identities; `distinct` has four alternatives using different chain pairs; `changed` has four alternatives that first make different unrelated context bindings, preventing exact-state reuse. Distinct alternatives leave unselected leaves unknown. Every alternative leaves a distinct tag; optional consuming work removes a token separately in each alternative. This is complete source execution with raw alternative multiplicity, not an isolated equality service.

The shared family leaves the unrelated context variable unknown; changed binds it to its branch tag before common equality. Thus this contrast deliberately changes the cache key's validity state while retaining the common equality problem. It does not claim those two source families have identical answers.

Prepared rules are reused across changed queries alternating depth n/n+1 and insertion order. Source and exact-query validation charge their actual costs. Compare seven paths: ordinary contextual, shared-deduction contextual, corrected relational, compiled scan, compiled indexing, inferred specialization and exact-schema answer generation. The exact-source control validates all rules/query fields; it is not a general compiler or an automatic hoisting transformation.

H1: compatible substantive repeated deductions can repay retention. H2: single/distinct requests and intervening bindings expose costs without reuse. H3: map retention can introduce copying that defeats saved equation work. H4: eliminating source work entirely can remain stronger than both execution organizations on these schemas.

The runner gate compares complete answers in all modes with the independent scalar evaluator at depths 0/1/8 and both resource settings, plus interrupted then reused preparation. Exact-control near misses reject altered source, extra/missing resources, missing outputs and changed input shapes. A structural test on the actual measured schemas verifies newly computed transition counts: n+1 for single/shared, 4(n+1) for distinct, and 4(n+2) for changed. Test-only retained observers do not run in cost binaries.

## Accounting and exploratory runs

CLI: `s02_deduction MODE FAMILY DEPTH QUERIES RESOURCE REVERSE_FIRST [CANCEL_TICKS]`. Primary total includes preparation, setup, execution with complete observation, engine/answer disposal and prepared disposal. Source/input construction and first-answer latency are separate. Cache retention ends with its arena owners; query and prepared disposal must restore requested-allocation baselines. Validate complete answers outside primary intervals. Native compilation is outside this measurement.

Use ordinary-allocator release binaries with engine/kernel/observer counters disabled. Allocation diagnostics use a separate meter binary and measure requested bytes, not RSS. Freeze source/binary hashes and toolchain before comparative runs.

Cross all seven modes, four families, depths 0/8/32, one/four queries, resources absent/present and both starting orders: **672 exploratory single timings**, randomized with seed 7211. No gain/loss classification is supported by these single timings.

Allocation covers short/cold (depth0, one query) and substantive/reused (depth32, four queries), all modes/families/resource settings/orders twice: **448 processes**, with exact per-cell replay. Cancellation uses shared/depth8/two queries/resources/forward, cancelling the first query after zero/one ticks, all modes and both allocator builds: **28 processes**. Total **1,148**, plus meter self-check.

Each process uses the first available CPU, 60 seconds wall/CPU, 1 GiB address space and the existing two-million-step source/scalar limit. Preserve every failure or cutoff and diagnose it before drawing conclusions. Register confirmation separately after inspecting consequential costs and controls. Sizing cannot select an architecture or resolve broader integration mechanisms.
