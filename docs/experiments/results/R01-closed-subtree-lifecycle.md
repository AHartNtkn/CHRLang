# Immutable-subtree correction: paired lifecycle evidence

The correction substantially reduces Specialized lifecycle time in four carry-heavy families. The other four Specialized comparisons and all Inline comparisons have overlapping observed ranges. This closes T044's causal correction; it does not establish general architectural superiority. T045 selects checked finite recursive-relation eligibility and correspondence before native compilation.

## Measurement and validation

The [prospective registration](../registrations/R01-closed-subtree-lifecycle.md) compares baseline `a852a82` with corrected source `8b60e95`, using eight unchanged finite R08 fixtures and balanced worker controls. Eighteen configurations per version each receive one warmup, five primary repetitions, one allocation run and one work run: **288 attempted and validated processes**, **36 complete primary cells**, and **9,664 validated full observations**. Source application and raw/unique observation counts match across versions. No errors, timeouts, cutoffs or missing runs occurred. The longest process wall time was 0.111 seconds, below the registered 30-second bound.

Primary builds disable diagnostics and use the ordinary allocator. Separate builds measure requested heap and work. Source preparation, query construction, execution, complete unique observation, shutdown and disposal are charged; validation occurs outside measured intervals. Output disposal is included after validation. Native compilation, process startup, fixture generation and owner-thread teardown are excluded. These are cold query lifecycle results, not complete architectural lifecycle costs.

The seeded manifest interleaves both versions on three verified distinct physical cores. Frozen runtime sources, identical lifecycle examples, harness and binary hashes pass the live audit. Archive staging initially encountered an unsupported Python extraction API before any comparative process; explicit archive-member validation allowed staging with the installed Python. No comparative samples were discarded. [Audit](r01-closed-subtree-lifecycle/audit.json), [manifest and environment](r01-closed-subtree-lifecycle/metadata.json), [raw outcomes](r01-closed-subtree-lifecycle/runs.jsonl), [phase samples and summaries](r01-closed-subtree-lifecycle/summary.json), [source check](r01-closed-subtree-lifecycle/source-check.json). Current lifecycle semantic tests also pass with and without diagnostics: [default](r01-closed-subtree-lifecycle/semantic-default.log), [counter-free](r01-closed-subtree-lifecycle/semantic-off.log). Independent review audited the runner and the resulting interpretation.

## Timing results

Times are milliseconds, median [minimum–maximum] across five primary processes. Overlapping ranges are unresolved under the registration, not evidence of equality.

| Family | Baseline Specialized | Corrected Specialized | Corrected Inline |
|---|---:|---:|---:|
| one-zero | 0.104 [0.098–0.121] | 0.090 [0.076–0.141] | 0.076 [0.069–0.091] |
| one-work | 7.628 [7.503–8.210] | 0.549 [0.521–0.597] | 0.473 [0.443–0.530] |
| two-work | 32.862 [31.140–34.760] | 2.109 [1.792–2.187] | 1.026 [0.895–1.058] |
| owner-product | 5.200 [5.115–5.942] | 5.462 [5.371–5.646] | 1.832 [1.699–1.890] |
| asym-first | 67.853 [65.206–72.880] | 4.470 [4.342–5.237] | 0.506 [0.469–0.530] |
| asym-last | 70.259 [66.971–76.934] | 7.360 [6.703–7.625] | 0.504 [0.448–0.524] |
| duplicate-eight | 4.713 [4.566–4.971] | 4.548 [4.459–5.034] | 0.161 [0.133–0.212] |
| mixed-add-infer | 0.215 [0.195–0.249] | 0.188 [0.181–0.237] | 0.101 [0.082–0.122] |

The four carry-heavy Specialized before/after ranges are separated favorably. Cheap and publication-heavy cases do not resolve a timing benefit or regression. All Inline before/after ranges overlap, including small movements in either direction.

In this session's current alternatives, Inline and Specialized overlap on one-zero and one-work; Inline is faster with separated ranges on the other six families. This is not pure per-application executor attribution: one-work performs 513 applications in both organizations, while two-work performs 1,026 regionally versus 2,051 Specialized, and asym-first performs 516 versus 4,111. Factoring and source work remain distinct explanatory differences.

Balanced corrected Threads1 takes 1.417 [1.042–1.480] ms and Threads2 0.986 [0.907–1.380] ms. Threads2 overlaps both Inline and Threads1. There is no resolved current worker advantage and no basis for a transitive ranking from earlier sessions.

## Requested heap and necessary complexity

Closedness metadata is charged at node insertion and arena cloning. Inline allocation traffic increases by 72–2,096 requested bytes across the eight families. Specialized traffic decreases in every family; for one-work it falls from 4,106,957 to 557,934 bytes, and for two-work from 16,246,463 to 2,101,964 bytes. Peak requested heap can still increase in cheap/publication cases; owner-product rises from 3,764,352 to 3,771,269 bytes despite lower total traffic. Exact per-cell allocation and phase measurements remain in the summary.

All 32 nonthreaded allocation runs restore their starting requested-heap baseline. The four threaded runs retain 48 requested bytes each, consistent with the separately established receiving-thread lifetime effect. This residue remains reported and charged, without warming or subtraction. Requested allocation is not RSS.

The correction requires one immutable property maintained at arena insertion and copied with the arena. Its contrary semantic tests preserve dependencies through unknowns and aliases, late binding, branch-local bindings and speculative failure. It eliminates repeated structural discovery without changing source arbitration. These results justify the correction within its measured boundary; they do not justify further automatic executor tuning.

## Decision and next investigation

T044 is complete. T045 will test a sufficient certificate for finite recursive relations, initially sealed unary addition with a ground finite control argument and possibly nonground payload/result. It must preserve complete residual aliases, conflicting result equations and occurs-check failure, and reject unsupported control inputs distinctly from source failure. Preparation should be reusable across changed queries. Contextual interference witnesses are required because predicate closure alone does not justify eager execution.

This can change Q1/Q4/Q5/Q8 by establishing when CHR arbitration, matching and recursive store maintenance can be avoided. It has greater present value than another worker or executor tuning pass. Eligibility and correspondence precede code generation and comparative timing. If the certificate needs broad contextual or termination analysis, the already certified R04 finite fragment is the lower-cost alternative for measuring native compilation lifecycle. No language restriction is adopted, no workload weights are inferred, and the wider architecture goal remains active.
