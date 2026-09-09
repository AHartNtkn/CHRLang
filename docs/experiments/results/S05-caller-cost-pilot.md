# Call reuse saves repeated work, but does not settle the architecture

Checked call reuse reduces requested allocation when private keys repeat across different callers. Distinct keys and one-query lifetimes add overhead. The source-specific elimination control has the lowest median time in all 64 configurations, so this pilot supports investigating other ways to avoid execution before refining this cache further.

These are exploratory timings, not confirmed speed classifications. The research remains open, including broader call eligibility, effectful calls and cache lifetime.

## What was compared

The [prospective registration](../registrations/S05-caller-cost-pilot.md) fixes seven complete execution paths, 64 configurations, five ordinary timing repetitions, two allocation repetitions and 28 additional cancellation cells. Each query has two raw answers with jointly aliased fresh variables and a caller-specific residual marker. Prepared rules are reused while queries change. Private recursion either carries a result or constructs necessary nested output; the caller then performs separate recursion and observes that output twice.

`call-direct` and `call-memo` use the same checked private-phase boundary and complete caller replay. `direct` uses ordinary persistent execution; `whole` uses compact whole-state reuse; `scan` and `sealed` use generic and inferred-specialized compiled scanning. `lowered` is a hand-derived control for this exact source, with checked query shape and complete owned outputs. It is not a general compiler.

The primary endpoint sums preparation, query setup, execution with owned observation, caller disposal, consumer release and prepared disposal. Input construction is separately reported and also included in a second total. Compilation and process startup are not isolated. Each process performs full independent semantic validation and an unmeasured warm-up before fresh measured preparation; this is not a cold-allocator experiment.

## The audit passes

All **3,220 processes** are present: 2,268 ordinary and 952 allocation runs. The auditor checks all 476 mode/configuration cells, exact repetition identities, phase order, answer counts and first-observation bounds. All **476 allocation replays agree exactly**, adjacent phase live readings agree, and every final disposal restores its starting live baseline. No failure record exists.

All nine frozen source/binary hashes match the current files. The freeze identifies the base commit and hashes the new runner files separately; its empty tracked-source patch alone would not capture those new files. The measurement source has not changed during this audit. The independent scalar evaluator owns its bindings, rule matching and branch progression; full comparison preserves joint aliases, residuals and raw multiplicity. Its cutoff raises an error. Measurement itself checks deterministic counts rather than rerunning the oracle inside timing.

The 16 counter-free call semantic tests pass again, and scoped strict Clippy passes for the runner. Deliberately missing or duplicated records, incorrect answer counts, missing phases and altered allocation traffic are rejected by the auditor. These audit checks establish the recorded experiment's integrity, not broader source coverage.

## Repetition saves allocation; cache misses cost memory

Against the matched uncached call path, memoization requests fewer bytes in **all 16 repeated-key/eight-query configurations**. It requests more in **all 16 distinct-key/eight-query configurations** and **all 32 one-query configurations**. These are deterministic allocation comparisons, not counts of timing wins.

For eight queries with private depth 32, caller depth 8 and payload depth 8:

| Private computation and keys | Uncached requested bytes | Memoized requested bytes | Uncached peak growth | Memoized peak growth |
|---|---:|---:|---:|---:|
| Carrier, repeated | 858,180 | 473,627 | 27,128 | 27,312 |
| Carrier, distinct | 858,180 | 869,260 | 27,128 | 51,897 |
| Constructed output, repeated | 1,449,620 | 813,371 | 41,544 | 45,952 |
| Constructed output, distinct | 1,449,620 | 1,487,324 | 41,544 | 93,209 |

Requested traffic excludes input construction, matching the primary endpoint. Peak growth is the largest phase peak above the initial owner baseline, including input and retained answers; it is neither RSS nor a sum of phase peaks. The repeated constructed-output difference occurs in execution/observation: 1,438,768 versus 802,519 requested bytes. The cache reduces repeated private work but retains results, and distinct entries accumulate during the eight-query owner lifetime.

## Stronger controls change the interpretation

Here is the complete constructed-output/repeated-key example above. Times are microseconds for all eight queries and preparation through disposal; ranges contain the five samples.

| Complete path | Median time | Observed range | Requested bytes |
|---|---:|---:|---:|
| Ordinary persistent | 748.5 | 725.5–1,576.0 | 1,145,816 |
| Whole-state reuse | 7,733.1 | 7,021.6–11,650.3 | 8,505,632 |
| Generic compiled Scan | 990.3 | 857.6–1,049.6 | 1,147,364 |
| Specialized compiled Scan | 733.9 | 683.0–1,137.2 | 971,976 |
| Uncached private calls | 1,006.2 | 998.0–1,528.4 | 1,449,620 |
| Memoized private calls | 648.4 | 622.0–982.7 | 813,371 |
| Exact-source elimination | 114.1 | 94.6–231.0 | 119,304 |

Memoization's lower median than specialization in this example is not a confirmed ordering: their ranges overlap. Including input construction gives medians of 658.4 microseconds for memoization, 745.4 for specialization and 124.7 for exact-source elimination. The input-inclusive endpoint does not change this example's main interpretation.

The exact-source control has a lower median than all six other paths in every configuration. This is descriptive evidence for these contractible sources, not a compilation-inclusive architectural victory. It still constructs required output; eliminating interpreter work does not eliminate that obligation. A broad call-reuse conclusion needs sources where the strongest applicable lowering cannot remove the repeated computation so cheaply.

First-observation latency also exposes a cold/reuse distinction. In the constructed-output example, memoized setup-to-first-answer medians are 112.3 microseconds for query 1 and 47.3 for query 8; uncached values are 100.8 and 97.4. Preparation is separate. The caller implementation eagerly computes both private alternatives before serving the caller, so these numbers do not establish streaming private execution or bounded interruption within that expansion.

## Consequence and remaining challenge

Call-level reuse now has complete-caller cost evidence, whereas whole-state recognition can miss reuse across caller-specific state. The bounded allocation benefit is real. It does not establish that cache recognition and replay are the best architecture: ordinary execution, specialization and source elimination have different favorable conditions and obligations.

The strongest remaining challenge is economical reuse across broader valid boundaries and sustained lifetimes on work that survives applicable lowering. Required work includes effectful/resource-dependent admission, conservative-analysis false negatives, key construction and result transport, bounded caches with eviction/recomputation, and interruption during private work. These questions remain scheduled under S05 and S08.

Do not run a broad confirmation merely to classify the overlapping memoization/specialization timings here. Either ordering would leave both broad mechanisms open and retain exact-source elimination as a stronger control for this family. The next [breadth review](S05-caller-breadth-review.md) therefore selects compact structural solving, which can change whether enumeration is required at all. This is a priority judgment, not a rejection of reuse.

## Reproduction and evidence

Run `python3 research/chr-reuse/experiments/audit_call_cost.py` to regenerate the descriptive summary and audit receipt. The original pilot command is `python3 research/chr-reuse/experiments/call_cost_pilot.py`; it refuses to overwrite existing evidence. Frozen binaries are identified by hashes and their original temporary paths; those files are needed to repeat the hash check as written.

[Raw runs](s05-caller-cost-pilot/runs.jsonl), [all cell summaries](s05-caller-cost-pilot/summary.json), [audit receipt](s05-caller-cost-pilot/audit.json), [source/binary freeze](s05-caller-cost-pilot/freeze.sha256), [toolchain](s05-caller-cost-pilot/toolchain.txt) and [host](s05-caller-cost-pilot/host.txt) preserve the results and their scope.
