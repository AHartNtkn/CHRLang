# Integrated graph scanning is faster in the controlled cases despite higher heap traffic

Local graph scanning passes the registered practical-gain test against conventional Scan in all six width 64/four-query families on both controlled CPUs. Its larger heap traffic therefore does not imply slower execution. Retained joins have a narrower benefit and substantial setup costs; no complete architecture is selected.

## The complete comparison

The [pilot registration](../registrations/S02-multihead-lifecycle.md) compares seven existing execution paths across six source families, widths 4, 16 and 64 and one/four changing queries per prepared ruleset. The engine implementations are unchanged. The runner adds separately measured source construction and disposal to the already qualified preparation, query, execution, observation and disposal paths.

All 252 configurations reproduce the old allocation records in both repetitions after normalizing external live-byte baselines. All query, cancellation, prepared and source owners restore. The pilot then completes 1,260 ordinary-allocator, counter-free timing processes. Thus the timing findings are not an unexplained change to the work performed by the allocation control.

The unpinned five-sample ranges overlap in many comparisons. They supply 17 strong pilot separations for local graph scan against Scan and no losses, but cannot settle the other 19 cases. The clock probe's median empty interval is 13 ns in the ordinary build and 14 ns with allocation diagnostics; no overhead is subtracted. Larger variation needs a different explanation than timer-call overhead alone.

## Core placement was a consequential control

Local inspection finds CPU 0 with maximum 4.7 GHz/base 2.1 GHz and a hardware sibling, while CPU 8 reports maximum 3.4 GHz/base 1.5 GHz without one. The original runs could use CPUs 0–15, and did not record where each measurement executed. Different placement is a plausible contributor to the spread, not a retrospectively proven explanation of individual samples.

The [prospective placement study](../registrations/S02-multihead-affinity.md) reuses the frozen timing binary. It pins each child to CPU 0 or CPU 8, verifies affinity before execution, and changes no system power settings. Seventeen randomized blocks compare all seven engines within each family/core combination at width 64 and four queries: 1,428 additional processes. Reported frequencies are capabilities, not claims about a fixed clock during measurement.

Within-block timing ratios are tested for a median gain exceeding 10% or a loss exceeding 10%. Exact one-sided sign tests receive Holm correction across 192 directional tests for 96 comparisons. Those tests assume the blocks are representative despite possible thermal/frequency dependence. Their conclusions apply to these sizes, queries and cores.

| Family, width 64/four queries | Local graph scan, CPU 0 ms | Partial joins, CPU 0 ms | Conventional Scan, CPU 0 ms |
|---|---:|---:|---:|
| Sparse updates | 0.707 | 0.681 | 1.296 |
| Broad updates | 1.669 | 2.347 | 2.857 |
| Nested equality | 0.904 | 0.850 | 2.199 |
| Cold prefixes | 0.200 | 0.390 | 0.253 |
| Dense prefixes | 0.835 | 20.284 | 1.098 |
| Three-head consumption | 0.265 | 0.538 | 0.399 |

These are medians of complete non-cancellation phase sums. Local graph scan qualifies as a gain against conventional Scan in all six families on each CPU. That is a bounded performance finding for this integrated execution path, not a claim that all graph organizations beat all conventional compilers.

Retained joins do not uniformly improve on local graph scanning. Against that stronger graph control, full tuples have no qualified gains, eight losses and four unresolved comparisons across the two CPUs. Partial joins have one qualified gain, seven losses and four unresolved comparisons. The gain is nested equality on CPU 8, with a median paired ratio of 0.790. Sparse and nested comparisons on CPU 0 remain unresolved at the registered threshold; favorable medians alone are insufficient.

Against conventional Scan, full tuples have six gains and six losses. Partial joins have five gains, six losses and one unresolved result. Indexed and specialized variants remain in the data: specialization has no qualified gain or loss against Scan in these controlled cells, while indexing has adverse cases. These findings do not select a universal access or specialization policy.

## Memory and time favor different choices

In the nested four-query case, local graph scanning requests 4.26 MB and peaks at 130,331 bytes above baseline. Partial joins request 1.74 MB but peak at 231,571 bytes; Scan requests 2.57 MB and peaks at 89,874 bytes. Local graph scanning is faster than Scan in the controlled study while using more traffic and peak storage. Partial joins reduce traffic but retain more state, and their additional speed benefit depends on the tested core and comparison.

These dimensions cannot be collapsed into a winner without workload and resource requirements the investigation has not supplied. The result strengthens integrated graph scanning as a serious complete-path competitor and keeps conventional Scan as the lower-memory control in these examples.

## Dense setup explains a large retained-join loss

The dense partial-join case requests 17.79 MB, with about 17.61 MB during setup. In the CPU 0 placement study, median setup time across the four queries is 17.63 ms and median engine disposal is 1.61 ms, while measured execution is approximately 0.001 ms. Looking only at execution would conceal almost the entire cost. Full-tuple setup has the same problem; on cold prefixes it also materializes work that never becomes useful.

Source inspection agrees with this attribution: the current retained organizations construct candidate tuples or successful prefixes, environments, readiness state and subscriptions before servicing the query. Dense prefixes genuinely admit many extensions. This explains the measured organization; it does not prove an intrinsic lower bound for integrated equality or for every join strategy. Demand-sensitive retention, alternative head order and compact ownership remain required alternatives where they could change the decision.

No implementation defect is established merely by this loss. The current partial strategy intentionally retains successful prefixes; changing that policy needs a distinct correctness and cost comparison. Likewise, local graph scan's gain does not justify ignoring its memory costs or its unsupported language features.

## Validation and interpretation limits

The two studies complete 3,192 comparative processes: 504 allocation runs, 1,260 unpinned timings and 1,428 pinned timings. Independent audits verify coverage, randomized order, exact allocation repetition, old/new allocation correspondence, phase continuity, final ownership, unchanged state sizes across builds and frozen sources/binaries. All 75 containing-package tests pass, along with the 42-comparison runner smoke path. Scoped strict Clippy passes in ordinary and allocation configurations; formatting passes.

Every process first validates all changing queries against independent scalar semantics. The measured owned answer is checked outside the measured intervals. This warms the executed path. The family named cold describes unused matching prefixes, not an uninitialized CPU cache or cold process.

Primary totals include source construction, preparation, changing-query setup, execution, observation and owner disposal. They exclude startup, validation, reporting and between-phase harness bookkeeping. Cancellation after setup or one adapter call is reported separately; differing adapter granularity prevents treating that point as equivalent progress. Stack storage, code size, independent source-program compilation, sustained consumers and general search lifetimes are not fully accounted for. These measurements do not establish complete architectural lifecycle superiority.

## What happens next

T072 remains active. Extend controlled placement confirmation to the smaller widths and one-query cases before treating the broad pilot's unresolved cells as gains or losses. Register the extension prospectively, retain the frozen binary where possible and keep the same competent controls. This is now a lower-cost way to settle consequential preparation/scale crossovers than changing a representation while timing uncertainty remains.

Then reassess demand-sensitive retention and broader guard/search/context support against native identity work, finite solving, reuse and coherent architecture composition. The new timing evidence changes the standing of integrated graph scanning; it does not finish those directions or the research goal.

## Evidence

[Runner](../../../research/chr-relational/tests/multihead_lifecycle.rs), [pilot driver](../../../research/chr-relational/experiments/multihead_lifecycle.py), [pilot audit](s02-multihead-lifecycle/audit.json), [old/new allocation check](s02-multihead-lifecycle/allocation-audit.json), [pilot freeze](s02-multihead-lifecycle/freeze.json) and [clock probe](s02-multihead-lifecycle/time-clock.json) retain the first comparison. [Placement driver](../../../research/chr-relational/experiments/multihead_affinity.py), [placement audit](s02-multihead-affinity/audit.json), [randomized order](s02-multihead-affinity/order.json), [raw runs](s02-multihead-affinity/runs.jsonl) and [environment](s02-multihead-affinity/environment.json) retain the controlled follow-up. [Tests](s02-multihead-lifecycle-validation/tests.log), [ordinary Clippy](s02-multihead-lifecycle-validation/clippy.log) and [allocation Clippy](s02-multihead-lifecycle-validation/clippy-meter.log) record validation.
