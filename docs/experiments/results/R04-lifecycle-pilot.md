# R04 finite-solving lifecycle pilot

Native trailed finite solving has the lowest measured lifecycle in every registered cold and four-query cell, with more than 10% separation and nonoverlapping five-run ranges against each comparator. This supports eliminating source execution for the exact eligible finite relation. It does not select a general CHR architecture or establish native compilation amortization.

All 720 processes completed, checking 1,800 query lifecycles with full raw observations against independent exhaustive assignments. No cutoff, timeout, instrumentation error or validation failure occurred. The [prospective registration](../registrations/R04-lifecycle-pilot.md), [frozen sources and environment](R04-lifecycle-pilot/freeze.json), [raw records](R04-lifecycle-pilot/raw.jsonl) and [complete phase medians/ranges and RSS](R04-lifecycle-pilot/summary.tsv) are the evidence. Clock median was 19 ns; phases below 1.9 microseconds do not support directional conclusions.

## Complete lifecycle results

Milliseconds, medians of five serial CPU-pinned processes. Cold includes one prepared execution; reuse includes one preparation and four changed-query lifecycles. Each cell includes query setup, execution, full observation, result/query/answer disposal and prepared disposal. Columns refer to native AC, support-CNF Z3, conflict-CNF Z3, generated global scan, generic global scan and generated active indexed execution. Complete ranges are in the linked TSV; table rounding is not the comparison criterion.

| Family | n | Queries | Native | Support | Conflict | Generated global | Generic global | Active indexed |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| asymmetric | 4 | 1 | 0.086 | 10.135 | 10.349 | 0.449 | 0.456 | 1.620 |
| asymmetric | 4 | 4 | 0.175 | 11.966 | 11.961 | 1.062 | 1.098 | 5.061 |
| asymmetric | 8 | 1 | 1.653 | 24.145 | 23.977 | 7.525 | 7.981 | 301.186 |
| asymmetric | 8 | 4 | 4.047 | 45.718 | 47.016 | 21.912 | 22.660 | 1025.616 |
| chain | 4 | 1 | 0.032 | 9.301 | 9.432 | 0.279 | 0.283 | 1.946 |
| chain | 4 | 4 | 0.057 | 9.826 | 9.762 | 0.656 | 0.648 | 6.640 |
| chain | 8 | 1 | 0.060 | 9.776 | 9.680 | 0.800 | 0.789 | 363.067 |
| chain | 8 | 4 | 0.141 | 10.308 | 9.746 | 2.225 | 2.308 | 1253.347 |
| contradiction | 4 | 1 | 0.005 | 9.137 | 8.725 | 0.375 | 0.387 | 1.970 |
| contradiction | 4 | 4 | 0.008 | 9.361 | 8.732 | 0.842 | 0.842 | 7.054 |
| contradiction | 8 | 1 | 0.005 | 9.217 | 8.934 | 0.381 | 0.373 | 183.418 |
| contradiction | 8 | 4 | 0.009 | 9.568 | 9.236 | 1.003 | 1.091 | 659.863 |
| unsupported | 4 | 1 | 0.098 | 12.295 | 12.133 | 0.445 | 0.406 | 0.720 |
| unsupported | 4 | 4 | 0.218 | 16.225 | 16.520 | 0.978 | 0.939 | 2.097 |
| unsupported | 6 | 1 | 0.928 | 45.313 | 45.889 | 3.416 | 3.358 | 6.163 |
| unsupported | 6 | 4 | 2.140 | 95.714 | 95.017 | 8.355 | 8.561 | 20.053 |
| weak | 4 | 1 | 0.069 | 13.435 | 13.621 | 0.374 | 0.378 | 0.371 |
| weak | 4 | 4 | 0.140 | 21.401 | 20.992 | 0.972 | 0.920 | 1.229 |
| weak | 6 | 1 | 0.625 | 68.724 | 66.096 | 3.412 | 3.275 | 2.869 |
| weak | 6 | 4 | 1.623 | 161.086 | 162.393 | 8.548 | 8.444 | 12.768 |

## What explains the contrasts

The native relation path avoids source occurrence matching, propagation histories and branch-engine copies. Its necessary machinery is finite-domain eligibility, relation normalization, support arcs, a reversible domain trail and assignment-to-residual recovery. The separately checked support-CNF encoding matches initial arc-consistency strength, but these complete algorithms differ in search, models and enumeration. This is a whole-path comparison, not a controlled inference-only timing.

Boolean preparation is material: support preparation on the eight-variable chain is 7.89 ms cold, against a complete native lifecycle of 0.060 ms. Four queries amortize preparation but do not reverse the comparison. Enumeration is also consequential: weak n=6 support spends about 148.6 ms in execution over four queries, versus 161.1 ms total. Startup alone therefore does not explain its adverse result. Support/conflict differences do not establish a general encoding preference under the registered range and 10% rules.

Ordinary source scheduling has a much larger effect here than generated versus generic dispatch. Chain n=8 takes 363.1 ms cold and 1,253.3 ms reused under active indexed execution, versus 0.800 and 2.225 ms under generated global selection. This includes activation order and copied-frontier consequences; indexing alone is not the treatment. The registration explicitly records that reused givens enter behind existing activations. The large cold difference also occurs without supplied givens, so that queue detail cannot be its complete explanation. A causal source-search investigation is selected below.

Observation remains part of the result. Native chain n=8 reuse spends about 0.111 ms of its 0.141 ms lifecycle reconstructing full answers; weak n=6 spends about 1.196 of 1.623 ms. Native initial AC occurs in setup, while Z3 initial solving occurs in execution. Compare full first-answer latency and lifecycle, not those isolated phases. First-answer columns start at query request and exclude preparation; add preparation when assessing a cold first answer.

## Memory and lifetime

One separate RSS run per cell uses the same ordinary-allocator, engine/kernel-counter-free binary. Chain n=8 four-query maximum RSS is 5,504 KiB native, 33,152 KiB support, 5,632 KiB generated global and 252,568 KiB active indexed. Weak n=6 maxima are 6,144, 34,432, 9,156 and 10,480 KiB respectively. These are whole-process observations, including linked code, external Z3 allocations, allocator retention, fixtures/oracle and retained complete answer batches. They are not isolated live-heap estimates, repeated memory distributions or streaming-memory claims. Post-query and final RSS are recorded without arbitrary baseline subtraction; four queries do not prove absence of long-lived retention.

Cold source execution consumes an ordinary prepared branch directly. Reuse prepares an unexecuted source query once, then clones its state and posts changed givens. Independent tests compare fresh and reused raw answers and occurrence traces, including conflicting givens, hidden choices and early disposal. The reference interpreter is independent of these paths.

## Dispositions and next experiment

R04 now has cost evidence for compact three-value finite relations, including weak propagation, unsupported values, equality chains, asymmetry and an arc-consistent contradiction. Native finite lowering is a viable specialized path. General source programs, open constructor domains, nonmonotone guards, solver resource encodings and native compilation costs remain outside this conclusion. The external Boolean candidate is not competitive in these small complete-enumeration regimes; larger coupled search or a different observation contract would require its own decision rationale and registration.

Select T033: independently diagnose source scheduling and frontier retention on the existing cold chain, contradiction and weak controls before choosing a storage or conditional-search comparison. Record source applications, splits, peak frontier and retained branch state in separate diagnostic builds; preserve primary timing builds. Distinguish early rejection, search width and answer-reconstruction cost, then register any comparative change only after source/fairness validation. This can change whether R03 needs conditional sharing, a trail, or simply an effective failure-service schedule. It is cheaper and more causally constrained than the strongest ready alternative, a full conditional resource/publication protocol. More Boolean timing repetitions cannot explain the observed orders-of-magnitude source-search difference.

R02 integration, conditional execution, static eligibility and language tradeoffs, compilation, longer lifetimes and parallel ownership remain open directions in the decision map. No workload weights or overall winner are adopted.

## Validation

Workspace all-target tests, Clippy with warnings denied and formatting pass. The finite experiment runner tests all five families and six variants in cold/reuse modes. Compiled query-template tests pass with counters enabled and disabled. Source/binary hashes were frozen before the randomized 720-process run. Validation and fixture construction occur outside primary measured intervals; full answer disposal is charged afterward. Rust/native compilation, process startup and dynamic-library loading are not credibly isolated per ruleset.
