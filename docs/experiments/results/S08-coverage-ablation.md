# Coverage ablation: matching overhead is real, but does not explain the crossover

**Equality-only coverage retains the large pure-source benefit, but the small consuming-resource regression persists.** The constructor-matching precheck adds allocations and application work on these sources. It does not account for most of the application-work increase. Keep the alternatives explicit; neither coverage configuration becomes a default.

This follows the [complete lifecycle pilot](S08-continuing-lifecycle.md). The experiment separates two responsibilities: maintaining a union of binding regions for equality, and consulting that union before constructor matching. The existing strong control has neither; `equality-binding-coverage` enables the first, and `matching-binding-coverage` enables both. Frozen earlier coverage binaries contain both. All variants preserve the same source and complete observations.

## The causal result

At 32 consuming-resource answers, body work falls from 28,444 calls without coverage to 9,425 with equality coverage. But application work rises:

| Configuration | Application-stage calls | Total service calls |
|---|---:|---:|
| Existing strong control | 35,473 | 91,255 |
| Equality coverage only | 54,181 | 89,467 |
| Equality and matching coverage | 55,897 | 91,183 |

Matching coverage adds 1,716 application calls here. Most of the increase is already present with equality coverage alone. The hypothesis that the added matching check explains the main regression is therefore unsupported. Application includes more than matching; the next conditional attribution must distinguish candidate handling, job restarts and support operations before choosing a repair. Faster body progress can change their interaction, but these counters do not identify that mechanism by themselves.

At 128 resource answers, equality-only makes 3,460,829 total calls versus 3,486,673 with matching coverage and 4,411,985 for the strong control. Pure-source equality-only and both variants have identical stage counts and allocations: these sources do not exercise a useful matching-coverage distinction.

## Complete costs preserve the crossover

Each elapsed value is a median of seven ordinary, counters-off processes. Each process includes preparation, two continuing queries, delivery, all-answer retention, cancellation and disposal. The figures are descriptive pilot estimates; their paired ranges are retained in [the audit](s08-coverage-ablation/audit.json).

| Source / answers per query | Control | Equality only | Equality + matching | Equality-only / control paired median |
|---|---:|---:|---:|---:|
| Pure / 32 | 11.86 ms | 7.59 ms | 7.43 ms | 0.640 |
| Pure / 128 | 335.74 ms | 146.62 ms | 152.43 ms | 0.453 |
| Resource / 32 | 24.49 ms | 25.90 ms | 29.49 ms | 1.121 |
| Resource / 128 | 1,076.11 ms | 957.86 ms | 960.07 ms | 0.901 |

Equality-only versus both has a resource32 paired median ratio of 0.917, but its individual pairs range from 0.771 to 1.500. This pilot does not establish a reliable elapsed advantage for that distinction. Equality-only versus the control remains adverse at resource32 by a paired median 2.95 ms, exceeding the 1-ms signal floor; the seven pairs include one favorable sample. At resource128 it saves a paired median 104.81 ms.

Allocation diagnostics are exact across repetitions. Resource32 requests 14,776,564 bytes for the control, 14,850,336 for equality-only and 15,175,680 for both. Resource128 requests 604,561,124, 449,371,968 and 453,474,160 respectively. Matching coverage therefore has a concrete allocation cost on these sources even where elapsed differences remain uncertain.

All 120 lifecycle and 12 diagnostic processes pass. Every measured lifecycle returns to its initial requested live heap. Eight rebuilt control cells reproduce the prior pilot's allocation records exactly relative to their initial live heap; absolute starting offsets equal the changed executable-path string length, independently checked by the auditor. No algorithmic baseline was added. Independent equality, partial-region, matching/resource and targeted coverage tests pass in both feature configurations; Clippy passes for both.

## What this changes in the sequence

This ablation answers the immediate attribution question: separating the matching check saves some work and traffic, but does not repair the small-resource crossover. A broader coverage policy still needs nonempty/partial-region cost sources and application-stage attribution. Those are concrete required experiments, not semantic restrictions on fixtures.

The next selected implementation is shared completed-graph traversal. It has a diagnosed common-chain repetition and can change the architectural interpretation of templates versus dependency execution. Further conditional micro-attribution is less valuable immediately because the ablation already prevents an unjustified default choice, while it does not yet identify a replacement application plan. Partner ordering/indexing and broader integrated equality/matching/consumption remain the strongest distinct alternatives; reconsider them at the graph correctness/work gate. Two packages have completed since the [full portfolio review](S08-continuing-selection-review.md); conduct that review again within four. T074 and the goal remain active.

[Prospective registration](../registrations/S08-coverage-ablation.md), [frozen inputs and raw receipts](s08-coverage-ablation/), and [audit script](../../../research/chr-reuse/experiments/audit_coverage_ablation.py) supply the reproducible record. Run `PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/experiments/audit_coverage_ablation.py` to verify it.
