# Stable equation reuse now preserves complete source execution

Exact-context and dependency-valid caches run inside the existing scalar source machine and preserve its independently specified answers and progress. Cached success can enable consuming matches; cached failure does not fail another branch. This establishes a complete correctness path for cost investigation, not a performance advantage over direct sharing.

## Integration and ownership

`Machine::step_with_equation` intercepts only a pending equation over the machine's shared, append-only constructor arena. All other source transitions use the existing machine. A privately constructed `EquationAccess` carries the arena owner, operand handles and the branch's binding map into the cache. It avoids resolved-tree export and owned substitution import.

The standalone [kernel gate](S05-stable-kernel-gate.md) and source executor now use the same `EquationCache`. The cache accepts exactly one arena owner and retains its token, so equal numeric indices from another machine cannot pass as the same constructors. Clearing cache entries does not authorize a different arena.

**Source cursors now enforce their machine ownership.** Inspection found that the previous interface documented that requirement but did not reject a foreign cursor. A test demonstrated the gap before the fix. Cursors now retain their owner, and source service and cursor inspection check it before interpreting handles. This ownership cost belongs in the upcoming comparison.

Source fresh-variable numbers remain local to their branch. Reusing an equation result across equivalent numeric inputs does not identify variables between branches: validity is checked against the receiving binding map, and replay changes only that map. Consumption, propagation history, pending work, occurrence identities and raw branch multiplicity remain ordinary source responsibilities. The cache does not merge states.

## Eligibility and failure behavior

The scalar machine rediscovers eligible applications by scanning once pending effects drain. Applying cached bindings therefore uses its normal subsequent eligibility check. It does not need an indexed wake-up list; indexed invalidation remains a separate integration option. The operation cache still returns changed variables for an executor that does need them.

Source failure remains a branch event. A cached clash returns failure for that equation's branch; the FIFO scheduler continues servicing siblings. Successful hits replay bindings before further source work. The raw-answer wrapper preserves duplicate alternatives and reports exhaustion separately from spending an advance budget.

## Independent gate

The source executor passes **576 configurations**: 64 registered semantic/application sources, three policies and capacities 0, 1 and 32. These check full expected observations, joint aliases, raw multiplicity, exhaustion and finite prefixes beside continuing work. The registry includes consumption, propagation history, fresh locals, recursive computation and finite-sibling cases. Capacity zero and one force recomputation and eviction rather than assuming indefinite retention.

Additional directed sources are compared with the independent owned-syntax evaluator:

- Two equivalent branch equations produce cache hits whose bindings enable consumption; both raw answers remain observable.
- Repeated off-output clashes hit the cache while an independent finite sibling survives.
- A binding change makes an earlier successful equation cyclic; reuse is invalidated and the occurs-check failure is preserved.

Separate tests reject a foreign cursor before equation service and reject another machine's constructor numbers when a cache is reused incorrectly. The original 3,888 independent kernel comparisons continue to pass after extracting the common cache implementation.

Deliberately omitting successful replay or treating a cached failure as success makes the source gate fail. [Fault receipts](s05-stable-source/) record these semantic failures. The reference interpreter is unchanged.

## Next comparison and important limits

The next work is a prospective lifecycle pilot with repeated success and clash before and after discrimination, substantive structures, unique/trivial equations, binding invalidation and unrelated updates. Include ordinary scalar execution, the cache-intercepted Direct control, a competent compiled control and the strongest applicable direct-sharing executor. The intercepted Direct path still constructs change reports; ordinary scalar execution is necessary to price that interface overhead rather than granting it to the control for free.

Register exact source and reuse configurations only after the measurement harness and controls pass complete observations. Account for ruleset preparation, query-owned arena/cache creation, lookup and proof construction, execution, first/full publication, cancellation and disposal. The current source constructor builds a machine for each query; prepared-rules reuse and its actual setup costs must be explicit, not assumed from the presence of a cache. Cross-query cache ownership is not established by this gate.

The common arena remains retained until machine disposal. Bounded entry count does not bound that arena, and dependency gathering on misses still traverses the reachable input closure. The proof deliberately over-approximates variables needed for an early failure. These costs and potential improvements remain investigative questions.

No source timing matrix has run. T067 remains active for prospective costs and the direct-sharing comparison; generalized continuation tables and the broader architecture sequence remain open.

## Validation

[Validation commands and output](s05-stable-source/validation.json) cover default regressions for reuse/persistence, counter-free release kernel/source tests, strict all-feature Clippy and formatting. [Source hashes](s05-stable-source/sources.json) identify the tested implementation. The ownership change is included in these regressions; earlier measured binaries remain tied to their earlier source snapshots.
