# Mixed-source pilot: current native execution does not repay its cost

The combined native path takes longer than the fastest admitted Rust control on all 24 complete-query batches. Execution cost remains consequential after accounting for host overhead. This bounds the current implementation; independent user-program compilation and other graph organizations still require their own trials.

## What was compared

The [prospective registration](../registrations/S10-mixed-pilot.md) fixed all complete batches from the qualified common-source and substantive-source corpora: 24 batches, 213 queries. Each batch reuses prepared rules across changing queries. The common corpus exercises choices, correlation, replacement, bindings, failure, occurrence history and deterministic interactions. The four substantive batches cross early/late readiness and successful/failing siblings, varying useful work, irrelevant work and query identity within each batch. Exact programs are in [the frozen selection](s10-mixed-pilot/selection.json).

The native candidate is the qualified Python source frontend and emitter followed by the native HVM runtime and complete owned-answer publication. The Rust candidates are the existing 13 execution configurations. This is runtime execution of emitted programs; it does not measure independent machine-code compilation of each CHR user program.

All 24 batches admit Rust modes 0–9 and the native path. Prefix lowering admits three batches, finite solving one, and checked finite execution three. The 65 unsupported candidate/batch pairs are recorded as capability exclusions. Two ongoing-only common-source groups remain progress and lifetime obligations outside this completion-time pilot.

The primary interval starts before process invocation and ends after captured output and process exit. It charges startup, source input, preparation, queries, complete observation, publication and disposal. Internal service and lifecycle phases provide attribution. Native phases are nested within host time, and internal first-answer timestamps are not external latency. Runtime and user-program compilation costs remain unmeasured.

## What the measurements say

All 24 comparisons meet the registered loss screen on both CPUs: native median at least 10% higher with nonoverlapping five-sample ranges. Native service alone also loses on all 24 against the fastest admitted Rust service control. These are exploratory comparisons on the frozen batches, not significance tests or predictions for unseen programs.

The table reports pooled ten-sample process medians in milliseconds. Its Rust control is the fastest admitted pooled median for that batch—an exploratory oracle, not an implemented source-selection policy. Small differences between Rust configurations do not establish an adaptive policy or a reliable fine ranking.

| Corpus/group | Queries | Rust mode | Rust ms | Native ms | Native/Rust |
|---|---:|---:|---:|---:|---:|
| common 0 | 4 | 9 | 1.942 | 50.059 | 25.78× |
| common 1 | 4 | 4 | 1.953 | 49.659 | 25.43× |
| common 2 | 4 | 8 | 1.788 | 47.395 | 26.51× |
| common 3 | 4 | 7 | 1.896 | 52.248 | 27.56× |
| common 4 | 4 | 6 | 2.010 | 50.803 | 25.28× |
| common 5 | 4 | 3 | 1.846 | 49.903 | 27.03× |
| common 6 | 4 | 5 | 1.923 | 49.184 | 25.58× |
| common 7 | 4 | 6 | 1.845 | 49.347 | 26.74× |
| common 8 | 4 | 1 | 1.866 | 47.103 | 25.25× |
| common 9 | 4 | 5 | 1.889 | 46.389 | 24.56× |
| common 12 | 8 | 4 | 1.993 | 51.146 | 25.66× |
| common 13 | 6 | 2 | 1.958 | 47.298 | 24.16× |
| common 14 | 6 | 8 | 1.947 | 49.193 | 25.26× |
| common 15 | 5 | 6 | 1.901 | 48.164 | 25.34× |
| common 16 | 12 | 8 | 1.927 | 50.288 | 26.10× |
| common 17 | 12 | 5 | 1.995 | 50.834 | 25.48× |
| common 18 | 6 | 2 | 2.012 | 48.548 | 24.13× |
| common 19 | 6 | 6 | 1.911 | 50.400 | 26.38× |
| common 20 | 8 | 6 | 1.868 | 56.736 | 30.37× |
| common 21 | 8 | 8 | 1.940 | 53.630 | 27.65× |
| substantive 0 | 24 | 4 | 5.039 | 564.031 | 111.93× |
| substantive 1 | 24 | 4 | 2.845 | 158.478 | 55.70× |
| substantive 2 | 24 | 5 | 3.557 | 183.157 | 51.49× |
| substantive 3 | 24 | 6 | 2.877 | 154.906 | 53.84× |

Modes: 0/1 generic scanned/indexed execution; 2/3 specialized scanned/indexed execution; 4 contextual execution; 5 contextual with shared deductions; 6 persistent equality; 7 persistent equality with shared deductions; 8 resumable contextual execution; 9 direct conditional execution; 10 prepared prefix lowering; 11 finite solving; 12 checked finite execution. The [analysis](s10-mixed-pilot/analysis.json) contains admitted configurations, both CPUs' ranges and service comparisons for every batch.

## Why startup tuning cannot explain away the substantive results

On each substantive batch, the smallest native service observation exceeds the largest competing Rust whole-process observation on both CPUs. Even eliminating every native non-service cost would therefore leave these particular comparisons unfavorable. This is a lower-bound argument from the observations, not a measurement of an optimized implementation.

| Substantive group | Readiness / sibling | Rust whole process ms | Native service ms | Native whole process ms |
|---|---|---:|---:|---:|
| 0 | early / successful | 5.039 | 507.044 | 564.031 |
| 1 | early / failing | 2.845 | 103.436 | 158.478 |
| 2 | late / successful | 3.557 | 129.212 | 183.157 |
| 3 | late / failing | 2.877 | 99.498 | 154.906 |

Small common batches have substantial time outside the host's internal session interval. For common group 0, the median whole process is 50.059 ms, host session 7.486 ms and native service 0.520 ms. The median per-run difference outside the session is 42.422 ms. That difference includes launch, startup/imports, diagnostic reporting and exit; it does not isolate imports. Phase medians need not sum. See [attribution](s10-mixed-pilot/attribution.json) for all batches.

The current runtime's representation and evaluation costs remain possible targets for a distinct mechanism trial. The measurements do not distinguish intrinsic graph obligations from every correctable implementation choice. They do establish that host-only refinement cannot reverse the substantive comparison without changing execution as well.

## Evidence and limits

Eligibility ran 336 processes. The 271 admitted pairs then ran one excluded warmup and five measured repetitions on each of CPUs 0 and 1: 542 warmups and 2,710 measured processes. Randomized order used seed 20260917. All 3,588 processes finished within the registered bounds; complete admitted answers were checked outside the measured intervals, including aliases, raw multiplicity and exhaustion. Native temporary-artifact cleanup was checked after each run.

Primary binaries use ordinary allocation and disabled diagnostic engine/serialization counters. Operational budgets and identity counters remain. Separate [allocation](S10-native-allocation.md) and [residency](S10-residency.md) experiments retain their own scopes; their figures are not timing samples or evidence of continuous peak RSS. This pilot does not settle sustained reuse, ongoing siblings, richer source constructs, local graph ownership, parallel execution or compilation amortization.

Reproduce the descriptive analysis with `python research/chr-hvm/mixed_pilot/analyze.py`, then `python research/chr-hvm/mixed_pilot/attribute.py`. The former verifies frozen hashes before analysis. The [runner](../../../research/chr-hvm/mixed_pilot/run.py), [validation receipt](s10-mixed-pilot/validation.json), [eligibility outcomes](s10-mixed-pilot/eligibility.jsonl), and [raw runs](s10-mixed-pilot/runs.jsonl) preserve inputs, outputs, phase records and binary identity. Running the timing runner again writes a new experiment, so preserve this receipt first.

## Architectural consequence and next investigation

Carry forward the bounded loss of this complete native path. Do not select it on a claim that graph execution already repays its costs here. Do not generalize that result to independently compiled user programs, demand-driven graphs, fresh-derivation reuse or native local resource ownership: those mechanisms have different unresolved propositions.

Select an independent user-program compilation entry next under T073. Compare generated execution with prepared-data execution of the same source-derived plan, then the strongest applicable existing control. This can test whether interpretation responsibilities disappear, rather than reducing process overhead. The first package must identify the exact source fragment, plan, compiler path, favorable work-elimination witness, adverse short-query witness and independent complete-answer oracle before implementation or timing registration.

The strongest ready alternative is the qualified learning cost pilot: it could determine whether avoided search repays failure recognition and retention, but still needs substantive common-prefix sources. Compilation requires a new backend/measurement entry and is likely more implementation work; it has greater breadth at this boundary because generation, compiler cost and artifact lifecycle remain directly unmeasured. This is an investigation priority, not evidence that compilation wins. Review learning again at the compilation semantic/accounting entry or an obstruction, before enlarging compilation into a cost campaign. Integrated execution and distinct graph mechanisms remain next in the wider sequence.

If the compilation entry cannot isolate the same plan or a credible benefit, resolve that specific obstruction and reconsider learning or direct resource derivations. A valid generated candidate proceeds to prospectively registered sizing, compilation/reuse crossovers and adverse tests. A gain requires lifetime and capability challenges; a loss requires consequential execution attribution; overlap or cutoffs require further discriminating evidence. T078's broader composition and lifetime work remains unfinished. The research goal remains active.
