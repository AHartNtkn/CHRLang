# Partial joins avoid cold products, but conventional scanning remains a strong control

Partial joins lower allocation on sparse and nested updates, while cold and dense prefixes favor conventional scanning. Every measured owner releases correctly. These allocation results justify conditional comparisons, not an integrated-architecture winner; the breadth review selects source-derived resource-aware lowering next.

## Comparison and validation

The [registered allocation gate](../registrations/S02-multihead-ownership.md) completes 504 measured processes over 252 configurations, preceded by 126 independent-source preflights. All 252 repetition pairs match exactly. A further 252 allocation replays match after adding an argument-free semantic smoke path for test-runner integration: 882 allocation/preflight/replay processes total.

Seven candidates execute identical sources: direct graph scanning, full-tuple activation, partial joins, conventional Scan/Indexed and source-specialized Scan/Indexed. Families cover sparse, broad, nested, cold, dense and three-head consuming queries at widths 4/16/64, with one or four changed queries per prepared owner. Variable identities and a residual seed change between queries.

Every process validates complete raw answers with the independent scalar evaluator before measurement and validates the actual measured observation outside phases. Preparation, input construction, setup, execution, owned observation and disposal are measured separately. Cancellation after setup and after one adapter service unit is separate from complete-query traffic. Service units differ across adapters, so cancellation supports ownership evidence only.

All complete queries and both cancellation points restore the prepared owner's requested-live baseline. Prepared disposal restores the initial baseline. The independent audit reconstructs cell coverage, randomized order, command arguments, phase order, exact pairs and live continuity from receipts. Observation allocation agrees across all seven candidates in all 36 source/size/reuse groups, helping separate matching-state costs from returned-answer construction.

## Requested allocation for four queries at width 64

Values are decimal MB, including preparation, query inputs and disposal; cancellation is excluded from these totals. No workloads are weighted.

| Family | Graph scan | Full tuples | Partial joins | Scan | Indexed | Specialized Scan | Specialized Indexed |
|---|---:|---:|---:|---:|---:|---:|---:|
| Sparse | 2.354 | 1.268 | 1.197 | 1.247 | 1.658 | 1.535 | 1.946 |
| Broad aliases | 4.315 | 2.911 | 2.971 | 2.495 | 3.302 | 2.963 | 3.770 |
| Nested equality | 4.255 | 1.816 | 1.743 | 2.569 | 3.091 | 3.114 | 3.636 |
| Cold prefixes | 0.592 | 14.187 | 0.947 | 0.410 | 0.631 | 0.411 | 0.632 |
| Dense prefixes | 5.302 | 17.170 | 17.789 | 1.059 | 1.308 | 1.060 | 1.309 |
| Three-head consumption | 0.693 | 0.945 | 0.984 | 0.549 | 0.642 | 0.550 | 0.643 |

Partial joins preserve a useful regime on nested equality and a much smaller traffic advantage over Scan on sparse updates. The cold case confirms that avoiding full-product materialization is necessary but insufficient: partial joins still allocate more than scanning when their suspended state never becomes useful.

Dense prefixes expose the cost of retaining every possible extension. Partial joins allocate 17.61 MB in query setup out of 17.79 MB total in the displayed case. Full tuples similarly allocate 16.99 MB during setup. Conventional Scan allocates 0.161 MB during setup and 0.717 MB during execution. A zero execution-allocation interval for a cached candidate does not mean it avoided the query work; much of that work happened during setup.

Peak requested memory also differs. For nested equality, partial joins peak at 229681 bytes above the initial baseline versus 87984 for Scan. For dense prefixes, the corresponding values are 2284634 versus 60203 bytes. These are measured lifecycle maxima, not sums of phase peaks. Lower traffic therefore does not imply lower retention or peak demand.

## What the result changes

Keep conventional Scan in every consequential comparison. The graph scan's larger traffic shows that direct handle matching alone does not guarantee efficiency. Full-tuple and partial-join caches have explicit state-building and retention liabilities. Partial joins address the witnessed cold-product defect but cannot avoid a product when early prefixes genuinely match.

These liabilities belong to the measured organizations. They do not establish an intrinsic lower bound for equality graphs or partial joins. More precise dependency ownership, compact environments, demand-sensitive caching or different head order could change costs, provided source priority and raw answers are preserved. Those alternatives remain unresolved rather than being rejected from this matrix.

The specialized controls are included even where they allocate more. Source specialization does not automatically improve requested traffic; its possible timing benefit is a separate question. No ordinary timing matrix ran here.

## Reproducibility and limits

The [audit](s02-multihead-ownership/audit.json), [runner replay](s02-multihead-ownership/runner-replay-audit.json), [binary/source freeze](s02-multihead-ownership/freeze.json), and individual receipts retain evidence. The initial measured runner and registration are archived. The current runner's argument-free path performs 42 independent source comparisons without invoking mixed-counter measurement; explicit measurement arguments still reject work counters. The 252 replay cells verify unchanged readings after that integration change.

All 61 relational tests pass in default and all-feature test configurations, with the allocation-only runners' separate checks and the new smoke path also passing. Scoped strict Clippy passes. Dependency build-script warnings are retained. No reference-interpreter or conventional-engine implementation changed.

Requested heap bytes are not RSS. Stack storage and code size are not allocation-meter coverage; source/process startup and compilation are not included. Observation is separately measured, but sustained consumers and general search lifetimes are not covered by these finite deterministic queries. No speed, compilation-inclusive or complete-architecture ranking follows.

The [four-package breadth review](S02-multihead-breadth-review.md) selects T073 source-derived resource-aware lowering. T072 retains timing, broader language support, alternative organizations and sustained lifetime. The architecture goal remains active.
