# Scheduled templates save memory against zero-follow reuse, but lose to simpler execution

**Scheduled templates request less memory and peak lower than zero-follow templates in all 96 source cases. They request more and peak higher than dependency execution, Direct and explicit carriers in every case.** Preserving scheduling therefore has a concrete implementation, but its retained plans are not yet justified by total cost.

The [registered campaign](../registrations/S06-scheduled-ownership.md) validates 2,304 fresh processes, 1,152 exact repetition/layout pairs and 288 exact cross-build explicit-control pairs. Complete answers pass the independent scalar evaluator; every measured session restores its initial live heap after consumer disposal. These are requested heap bytes, not RSS. Metered time is not evidence here.

## Complete requested ownership

Each comparison contains 96 source cases. Ratios are scheduled templates divided by the named control; ranges show every case, without workload weights.

| Control | Requested-byte ratio range | Peak-live ratio range | Direction across cases |
|---|---:|---:|---|
| Zero-follow templates, same build | 0.598–0.910 | 0.401–0.915 | Lower in all 96 |
| Dependency execution, same build | 1.340–3.542 | 1.630–5.431 | Higher in all 96 |
| Ordinary contracted templates, feature off | 1.719–7.676 | 1.861–4.536 | Higher in all 96 |
| Explicit inferred carriers | 1.272–11.312 | 1.100–8.712 | Higher in all 96 |
| Direct | 1.822–4.864 | 2.539–10.678 | Higher in all 96 |

Ordinary contraction is a cost control on these qualified sources, not a scheduling-equivalent replacement on arbitrary resource programs. The preceding scarce-token test establishes that distinction.

Preparation and both changing queries are charged. Query depth changes from n to n+1, rather than changing only an inert marker. Half the cases discard the first query after one source tick and then complete the second with the same prepared rules. All answers survive producer/preparation disposal before independent validation and consumer disposal. Sources include deterministic chains, repeated-call choices, distinct-call choices, failure, consumption and both insertion orders.

## Why scheduled plans beat zero-follow templates

A follow-up structural test uses the existing source builder with one result and a 32-step wait chain. Both variants return its exact expected answer. Zero-follow retains 34 templates, each keyed by a different suffix; scheduled derivation retains one template containing 33 followed calls. This explains a concrete source of repeated key/plan ownership in zero-follow mode. It does not assign all measured savings to that structure.

For the pure successful depth-32 deterministic chain, forward insertion and both queries complete:

| Mode | Total requested bytes | Peak live bytes | Preparation | Both setups | Both executions |
|---|---:|---:|---:|---:|---:|
| Scheduled templates | 574,754 | 127,234 | 4,346 | 30,408 | 540,000 |
| Zero-follow templates | 956,649 | 312,061 | 4,346 | 30,408 | 921,895 |
| Dependency | 166,549 | 25,766 | 4,346 | 30,408 | 131,795 |
| Direct | 125,687 | 15,351 | 1,428 | 36,653 | 87,606 |
| Sealed compiled | 77,819 | 16,119 | 6,106 | 38,747 | 32,966 |
| Inferred carriers | 52,959 | 15,824 | 7,830 | 38,747 | 6,382 |

Disposal is included; it requests zero additional bytes in this example. Query-time derivation belongs to execution in this runner. It is not free preparation or isolated compilation.

The feature itself has a measurable representation cost even without scheduled calls: dependency mode requests 80–160 additional bytes and peaks 56–104 bytes higher; zero-follow requests 256–5,488 additional bytes and peaks 200–1,752 higher. Explicit controls agree exactly across builds. Comparisons within the scheduled build and across feature-off controls are both retained.

## Decision

The [full portfolio review](S06-scheduled-ownership-review.md) retains T073 for bounded ordinary timing against the same credible controls. Work counts alone cannot establish whether saved matching repays the added allocation. This is the next factual question; do not tune the scheduled-body table before measuring its complete cost. Review count resets after considering all 57 questions. No architectural winner or language restriction is selected.

Reproduce with `python3 research/chr-reuse/experiments/scheduled_ownership.py --audit`. [All phase receipts and summaries](s06-scheduled-ownership/analysis.json), raw runs, source archive and binary hashes are retained. The follow-up structural check is `cargo test -p chr-direct-choice --features scheduled-templates --test scheduled_retention -- --nocapture`.
