# Recorded reads can recognize an equality deduction without rebuilding its key

The new lookup policy preserves the tested source behavior and avoids constructing a fresh relevant-state key on a demonstrated cache hit. It now needs ownership and cost measurement against rebuilt keys and recomputation. This correctness result establishes neither a speedup nor lower total memory.

## Why this experiment comes next

**The tested checkpoint regimes already have applicable cost evidence.** The [resident restoration pilot](S04-resident-cost-pilot.md) reports 1,344 processes and 192 exact allocation pairs, including checkpoint intervals 1/4/16, undo, Copy and Indexed/COW. A fresh [archive audit](s04-restoration-applicability/audit.json) reconstructs its original audit, summary and phase table byte-for-byte against all 118 frozen sources and four binary hashes. The current basic restoration source differs only by the reunion module declaration; its lifecycle runner is identical.

That pilot explicitly excludes source construction and validates answers before producer disposal. Those limits constrain its claim, but do not erase its observed large reconstruction costs. Adding an equal nonnegative source-construction cost to both paths cannot reverse their ordering, although it can narrow ratios. Cache and allocator effects from changing validation placement require measurement and are not covered by that arithmetic argument. No new timing is inferred from source similarity.

The pilot already identifies larger retained states, different switching policies and sustained lifetime as unresolved. Repeating its existing matrix would not answer those questions. The [relevant-key allocation attribution](S02-relevant-attribution.md), meanwhile, identifies a specific construction cost that could reverse some integration comparisons. That makes sound cheaper recognition the stronger next investigation. T077 remains unfinished; T072 is now active.

## What changes in the candidate

**The candidate checks a saved read set before allocating a new key.** It searches existing cache entries with the same ordered pair of input representatives. For each candidate it checks every saved value's current representative and complete constructor descriptions. A match replays the same local deduction as the rebuilt-key control. A miss constructs the original complete key, computes the deduction and records it under the existing 4,096-entry limit.

The saved reads cover the transitive constructor graph reachable from both inputs. If a newly reachable child appears, some checked constructor description must change. A changed descendant representative also fails validation. This is the soundness argument for checking the recorded closure instead of independently rebuilding the closure on every hit; the executable counterexamples test its consequential premises.

Ordered maps compare borrowed descriptions. The persistent-map API returns owned values, so that variant can still clone descriptions during validation. Both can inspect several cached variants with the same inputs before finding a match. These are real costs to measure, especially on near misses and low reuse. The candidate introduces a lookup policy, not a new executor, identity model or result semantics.

## What the gates establish

| Check | Evidence | Scope |
|---|---|---|
| Complete source correspondence | All 48 configurations agree with independent scalar answers and Scan/Indexed controls; contextual recomputation, exact reuse, rebuilt relevant keys and both new map variants participate. | Four source families, depths 0/4/16, resource present/absent and both query orders. |
| Useful cross-context reuse | Diagnostic assertions require changed-context hits separately in rebuilt-key, validated-read and persistent validated-read modes; the final-step counter correction now records 276 hits per mode across the changed-source cases. The initial frontier-based observation was 266; see the [counter-ownership result](S02-read-near-miss.md#counter-ownership-now-includes-the-final-step). | Recognition occurs in actual source execution, not only a cache API test. |
| Caller ownership and validity | Tests preserve different caller bindings and duplicate resource claims; incompatible descendants and cycles are rejected under both policies and both map representations. | These are semantic ownership checks, not heap measurements. |
| Key construction avoided | A repeated constructor merge produces the complete expected binding with zero key-construction scopes under read validation, versus two under rebuilt-key lookup. | A diagnostic operation witness; it does not measure total lookup/replay costs. |
| Counterexample sensitivity | An isolated mutation that validates only the two inputs incorrectly reuses an occurs failure. The descendant witness rejects it at `!valid.failed()`. | Demonstrates why descendant validation cannot be omitted. |

The initial extended test fails on the absent candidate entry points. After implementation, four counter-free relevant-deduction tests and five diagnostic tests pass. The contextual library/store/source/invalidation regression selection passes 23 tests, including the four counter-free relevant tests. Strict scoped Clippy passes. Final direct test processes run under 120-second wall/CPU and 1-GiB limits; both finish without a cutoff.

The mutation uses a temporary standalone source copy with the same local dependencies; it does not alter the production worktree or the reference interpreter. The independent scalar evaluator and compiled controls remain unchanged.

## Next: does recognition repay its costs?

**Qualify matched ownership and allocation before primary timing.** Extend the existing deduction lifecycle runner with the new policy, retaining contextual recomputation, rebuilt relevant keys, exact-state caching and applicable Scan/source-elimination controls. Reuse preparation across changing queries, validate retained answers after producer disposal, and exercise cancellation and low reuse.

Include many same-input near misses as a separate adverse case. The current cache orders by inputs and saved reads; validation can scan several variants where rebuilt-key lookup performs one tree search. Attribute successful validation, failed probes, miss construction, replay and retention. Compare ordered and persistent representations without calling persistent descriptor clones free. No gain follows merely from eliminating key-construction scopes.

If an avoidable dominant cost could reverse a consequential comparison, investigate it. If the measured total remains unfavorable under a competent implementation, preserve the bounded loss and its source scope. Broader local-rewrite integration, demand capability and compact solving remain required by the [sequence](../next-cycle.md). This gate does not resolve integration or the architecture goal.

## Evidence and reproduction

The [registration](../registrations/S02-read-validation.md), [bounded gate and source hashes](s02-read-validation/gate.json), [primary output](s02-read-validation/bounded-primary.log), [diagnostic output](s02-read-validation/bounded-diagnostic.log), [regressions](s02-read-validation/regression.log), [Clippy](s02-read-validation/clippy.log), [initial failure](s02-read-validation/red.log) and [mutation failure](s02-read-validation/mutation.log) record the result.

Run `python research/chr-relational/experiments/read_validation_gate.py` for bounded source validation and `python research/chr-relational/experiments/read_validation_mutation.py` for the isolated counterexample. The existing restoration receipts can be checked without rerunning measurements using `python research/chr-restoration/experiments/audit_resident_applicability.py`.
