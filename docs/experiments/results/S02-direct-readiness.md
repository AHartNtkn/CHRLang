# Direct readiness avoids work without changing selection

**Recognizing direct relevance makes the two width-128 readiness cases 39–40% faster over complete sessions. Compiled execution still takes about one-fifth as long.** The shortcut is a qualified experimental improvement, not a reason to select readiness everywhere or choose the relational architecture.

## What changed and why it is sound

The first queued equation can be serviced immediately when one of its current representatives occurs in a matcher-read column. That value is a seed of the existing relevance closure, and no earlier equation exists to take priority. Otherwise the full equation/constructor reachability path runs unchanged. This adds a root comparison while scanning read columns; it introduces no retained owner, source restriction or new scheduling contract. The feature `direct-readiness` is default off.

The new adversary puts an indirectly relevant equation before a later direct hit, with an optional genuinely unrelated prefix. It checks the exact first serviced equality, subsequent selection, consumed-source invalidation, complete exports and independent fork ownership. The fixture was corrected when its initial “unrelated” prefix shared a relevant endpoint. Both representations still need the full closure whenever the first equation lacks a direct witness.

Candidate and control each pass 27 store/source/library tests, including existing raw-source oracle checks for equality, constructors, guards, competing consumers, branches, cancellation and bounded interleaving. Scoped Clippy passes. The allocation comparison also checks identical advances, scope counts and all non-readiness allocations; this is stronger evidence of unchanged service than final answers alone.

## Ownership: where it helps

The [registered allocation campaign](../registrations/S02-direct-readiness.md) completes 144 processes across 36 configurations, with 72 exact meter/profile pairs and exact exclusive allocation sums. Two changing queries include preparation, setup, execution/observation and producer/preparation/consumer disposal. Both incidence representations are measured.

| Vector-incidence readiness case | Readiness requested bytes, closure → direct | Complete requested bytes, closure → direct |
|---|---:|---:|
| Broad128, reversed insertion/binding | 7,719,256 → 3,784,512 | 9,992,995 → 6,058,251 |
| Broad8, forward insertion/binding | 37,944 → 12,616 | 189,347 → 164,019 |
| Depth64 shared success | 341,616 → 337,928 | 878,467 → 874,779 |
| Depth64 separate failure | 1,136 → 1,136 | 245,667 → 245,667 |

Eight of twelve readiness configurations request fewer bytes; four are unchanged. Eleven peaks are unchanged; the set-incidence broad8 peak falls from 41,817 to 40,361 bytes. All full-settlement and compiled controls preserve complete requested bytes and peaks exactly. Allocation is requested heap ownership, not RSS. Instrumented clock values are not used for speed claims.

## Ordinary complete lifecycle timing

The separately [registered timing campaign](../registrations/S02-direct-readiness-timing.md) completes 1,200 processes and 12,000 sessions. Each process measures ten fresh prepared-rule sessions, each with two changing queries. Twenty paired blocks rotate five adjacent configurations; allocation and kernel counters are disabled. All complete answers are checked outside measured intervals. Compilation/source construction still require their separate architectural experiments.

Ratios are median paired process-mean complete times. **Bold** passes the registered gain gate: median at most 0.90 and every block below 1. Plain results are unresolved, not equivalent. “Direct” means the shortcut; all relational builds use stable candidates, borrowed cycles and vector incidence.

| Source | Direct / closure readiness | Compiled / direct readiness |
|---|---:|---:|
| Depth4, separate, success | 1.003 | **0.463** |
| 64, forward insertion, broad-cancel | 1.006 | **0.388** |
| 64, forward insertion, broad-forward | 0.675 | **0.312** |
| 64, forward insertion, broad-reverse | 0.768 | **0.280** |
| Depth64, separate, fail | 0.993 | 0.459 |
| 64, reverse insertion, broad-cancel | 1.002 | **0.351** |
| 64, reverse insertion, broad-forward | 0.774 | **0.279** |
| 64, reverse insertion, broad-reverse | 0.687 | **0.302** |
| Depth64, shared, clash | 0.994 | **0.038** |
| Depth64, shared, success | 1.017 | **0.047** |
| 128, forward insertion, broad-forward | **0.602** | **0.196** |
| 128, reverse insertion, broad-reverse | **0.606** | **0.196** |

The two qualified shortcut gains are width128; no shortcut contrast qualifies as a loss. Compiled qualifies against direct readiness in eleven of twelve cases. All twelve full-versus-full no-effect controls remain unresolved, with median ratios 0.986–1.032, consistent with their identical measured work and ownership.

The early-failure compiled comparison is unresolved because zero-based block16 has a 1.201 process-mean ratio. Its compiled sessions in microseconds are 311, 3243, 158, 187, 179, 177, 188, 191, 230, 224, versus readiness 564, 458, 381, 366, 382, 363, 360, 419, 454, 490. The median-session ratio is 0.473, but changing to that endpoint after seeing the data would not satisfy the registration. No sample is excluded. Width64 shortcut medians suggest savings but also contain contrary blocks. Neither uncertainty is used to choose a policy. The strong width128 gain and remaining compiled gap establish the mechanism's scoped result under the original gate.

## Decision and next investigation

Retain direct readiness as an experimental option. It removes a substantial, identified part of readiness work while preserving the ordering gate. Remaining closure work, broader constructor organizations, output-only matcher omission and state-inspecting guards still require investigation; this feature does not answer them.

The [full portfolio review](S02-direct-readiness-review.md) selects fresh graph memo/scratch attribution under T074. This tests costs of a competing graph organization after its existing context improvements, rather than assuming another local readiness change has higher value. T072 remains pending broader integration work. The review count resets; the architecture research remains active.

Evidence: [allocation analysis](s02-direct-readiness/comparison.json), [all 120 timing contrasts](s02-direct-readiness-timing/analysis.json), source/binary freezes and raw sessions in the corresponding directories. In the allocation comparison's reused schema, `conservative` is closure, `stable` is direct, and the nested `discovery` field contains the explicitly labeled readiness phase. Audit with `execution_attribution.py --direct-readiness --audit` and `stable_candidate_timing.py --direct-readiness --audit` in `research/chr-relational/experiments/`.
