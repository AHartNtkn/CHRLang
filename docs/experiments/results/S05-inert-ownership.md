# Separation reduces table allocation, but Direct remains substantially smaller

Inert residual separation reduces allocation versus existing whole-state tables on its favorable sources. It adds overhead on readable and late-bound callers, and every tested separated memo configuration allocates more than Direct. The finite owners release correctly; speed and sustained efficiency remain unresolved.

The [full breadth review](S05-inert-ownership-breadth-review.md) selects conditional equality/lifetime attribution next. Broader reuse costs and dependency questions remain required.

## What the comparison changes

**The benefit survives a stronger compact-key control.** Separated memo execution requests fewer bytes than CompactLive in 96 of 144 comparable scenarios and more in 48. Its relative peak live heap is lower in 84 and higher in 60. The 48 requested-byte losses are exactly the readable-caller and late-binding families, which gained no distinct-caller reuse in the source gate.

The earlier owned AlphaLive control gives the same 96/48 requested-byte split, but understates the contrary peak cases. In the early-failure family, compact keys have a lower peak than separated memo execution in 12 scenarios. The [supplementary registration](../registrations/S05-inert-ownership-compact-control.md) and measurements add this existing control before drawing a conclusion.

**Direct uses fewer requested bytes and lower peaks than separated memo execution in all 144 scenarios.** It likewise beats both whole-state table representations on these quantities. That does not establish a runtime winner: the source gate showed fewer executed transitions under reuse, while this experiment measures heap requests. It does establish that saved transitions have a substantial representation cost to repay.

Full exhaustion of four changed queries at depth four, with immediate answer release, illustrates the scale. Values are cumulative requested heap bytes for the charged lifecycle.

| Source | Direct | Compact whole-state table | Separated memo |
|---|---:|---:|---:|
| Inert caller tags | 87,156 | 422,928 | 236,732 |
| Readable caller tags | 102,974 | 468,114 | 505,850 |
| Late-bound variable observations | 108,702 | 514,298 | 566,138 |
| Same name, different readable arity | 90,344 | 426,116 | 239,864 |
| Dynamic duplicate residuals | 143,512 | 784,084 | 374,088 |
| Early failed branch | 56,910 | 249,414 | 227,738 |

For dynamic duplicate residuals, relative peak live heap is 11,369 bytes for Direct, 101,789 for compact whole-state tabling and 55,788 for separated memo execution. Separation improves that table's retention while remaining above ordinary execution.

**Separation without memoization is also an overhead in these fixtures.** It requests more bytes than Direct in all 144 scenarios; peaks are higher in 128 and equal in 16. It requests fewer bytes and has lower peaks than every table configuration. This control distinguishes extracting observations from retaining reusable transitions. The preceding slot-recycling repair is included.

## Where the allocation difference occurs

Preparation requests are identical across the modes for a given source. The larger differences occur during query service and observation. For the inert-tag example above, those phases request 74,776 bytes in Direct, 79,000 with uncached separation, 214,656 with separated memoization and 409,604 with owned whole-state tabling. This localizes the next attribution question to execution, key/edge retention and extraction rather than source preparation.

Readable and late-bound cases similarly add service allocation without new reuse. For readable callers, uncached separation requests 112,792 service bytes versus Direct's 88,920. For late-bound observations it requests 124,184 versus 93,976. The source implementation scans/exports potential residuals during service, making that a plausible repair target. These aggregate phases do not causally isolate scanning, term export, keys or edge payloads; a future ablation must do that before assigning savings to a specific repair.

## Every finite owner releases

All immediate-release queries return exactly to their prepared baseline after engine and input disposal. After prepared disposal only consumer-owned bytes remain; final consumer disposal restores the initial baseline. The matrix covers one/four changed query namespaces, full exhaustion/first-answer cancellation, and immediate/window-one/all consumers.

For dynamic residuals at depth four over four full queries, the consumer accounts are zero bytes for immediate release, 2,322 bytes for window-one and 17,040 bytes for retaining all eight answers. These outputs are checked again after producer disposal. Engine totals include active keys/nodes, cached edge data, the machine arena and per-derivation output owners together. Shared lifetimes prevent assigning individual live-byte totals to those subowners from these readings.

Prepared rules are reused, but transition tables remain query-owned. This is finite ownership evidence, not proof of bounded continuing cache memory, cross-query memo reuse or RSS behavior.

## Scope and validation

The [initial registration](../registrations/S05-inert-ownership.md) defines 576 configurations with two allocation runs each. The compact-key supplement adds 144 configurations and 288 runs. Final evidence therefore covers 720 configurations and 1,440 diagnostic processes. A full 1,152-process matrix from before a Clippy-only runner repair is also preserved; its normalized heap readings are identical to the final replay. In total, the audit checks 2,592 processes.

Every process performs full same-source preflight against Direct order and the independent scalar evaluator's raw multiset. Each measured owned answer is validated outside allocation intervals. Source construction, preparation, source disposal, query input/setup, service/observation, consumer operations and all disposal phases are charged. Oracle data and preallocated receipt bookkeeping are outside the baseline.

The build is release, no default diagnostic metrics, with the allocation meter enabled. Its self-check runs before each measurement. Each process has 60-second wall/CPU and 1 GiB address-space bounds, with 200,000 service calls per query. Exact phase readings repeat, and the [audit](../../../research/chr-reuse/experiments/audit_inert_ownership.py) checks commands, phase coverage, counts, retained consumers, source/binary hashes and all baseline endpoints. Strict Clippy passes for the current runner.

The [main matrix](s05-inert-ownership/final/audit.json) and [compact control](s05-inert-ownership/compact/audit.json) retain every configuration and phase. Prior runner sources and the previous Cargo manifest have snapshots alongside the receipts. The engine implementation, reference interpreter and independent evaluator are unchanged in this package. Requested allocations and relative live peaks are not RSS; this experiment makes no timing or compilation claim.

## Remaining investigation

A timing pilot must include Direct and compact keys, charge all lifecycle phases and preserve first-answer behavior. Extraction/key attribution could materially change that comparison, especially on no-reuse or variable-bearing sources. Variable-dependent residual separation, broader effect relevance, larger substantive reused work, cross-query lifetime and sustained consumers remain required.

The allocation result bounds this implementation rather than rejecting resumable reuse. It also does not justify combining it with other architectures without charging its source analysis, state boundaries and ownership duties. The next breadth decision below determines when to return to these questions.
