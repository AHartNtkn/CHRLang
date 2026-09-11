# Deep prefixes expose reusable matching work, with a strong invalidation cost

Proper intermediates avoid repeated deep prefix matching in the qualified source. Existing indexed execution still repeats that structural work. This supplies a meaningful future cost contrast, not evidence that retaining intermediates is faster or simpler.

The [registration](../registrations/S01-structural-prefix.md) defines 48 sources: widths four/eight, depths zero/eight/32, two changing query seeds, and sparse, keyed, broad-invalidation and deep-mismatch families. The [source test](../../../research/chr-relational/tests/intermediate_structure.rs) uses the existing engines and independent scalar executor. No comparative timings or allocations ran.

## What changes the architectural question

Each rule keeps two nested constructor matches and consumes a terminal occurrence. Proper intermediates retain the two-head match; scanning rediscovers it after each consumed terminal. Constructor depth varies the work in that saved match without disabling canonical identities or usable indexes.

At width eight on the sparse source, the observed counts are:

| Executor | Depth 0 | Depth 8 | Depth 32 |
|---|---:|---:|---:|
| Local scanning: pattern visits | 3,192 | 8,376 | 23,928 |
| Complete retained tuples: pattern visits | 1,608 | 9,800 | 34,376 |
| Partial joins: pattern visits | 656 | 1,232 | 2,960 |
| Proper intermediates: pattern visits | 2,616 | 3,192 | 4,920 |
| Conventional indexed: structural tests | 672 | 5,856 | 21,408 |

**Retaining a proper prefix reduces the depth-dependent matching work.** Each additional constructor adds 72 local pattern visits with proper intermediates versus 648 with scanning. Partial joins save the same prefix traversal but retain final extensions as well; proper intermediates still repeat more terminal matching. These are operation counts with different per-operation costs across engines, not interchangeable time units.

**Indexing helps candidate selection but leaves the deep traversal in this source.** At width eight, indexed execution visits 656 candidates versus scanning's 2,924; both counts remain unchanged as depth grows. Indexed structural tests still grow by 648 per added constructor. The initial prefix variables are unbound, and a successful candidate must traverse the constructor pattern to capture them. This result applies to the existing Global-policy access plan; it does not establish that a different activation or partner-order plan must repeat the same work.

**More retention is not automatically more reuse.** The complete-tuple strategy repeats prefix inspection for terminal combinations; at depth 32 it performs 34,376 pattern visits. Partial joins and proper intermediates have different retained-state and terminal-search costs. The [preceding lifecycle comparison](S01-intermediate-lifecycle.md) already shows that lower retained state can coexist with greater allocation traffic. This work screen cannot choose between them.

## Contrary cases and control limits

**Broad invalidation charges speculation with no useful join.** A higher-priority rule consumes every left occurrence first. At width eight/depth 32, local scanning makes eight pattern visits and conventional indexed execution makes eight structural tests. Proper intermediates make 2,384 pattern visits preparing work that is never used. This adverse source must remain in any cost comparison.

**Deep mismatch establishes that traversing a prefix is insufficient to fire.** Every middle input differs at the innermost constructor; no terminal is consumed and no output becomes `hit`. At positive depths, scanning, partial joins and proper intermediates perform the same pattern work in this source. The depth-zero negative case has no middle facts and is a semantic control, not a matched cost baseline.

**Inferred specialization does not apply to these rules.** All measured specialized-application counts are zero; its rows equal ordinary execution under the same access mode. The existing specialization checker requires a leading removed head, no kept heads, and at most one distinct nullary partner. It therefore supplies no additional execution mechanism for this three-head source. Independently generated Rust has not run here. Source inspection of the generator shows recursive constructor calls, but that is not a measured generated-control result.

The next cost study must first qualify actual generated execution and consider an eligible Active-policy or improved partner-order control. It must then charge preparation, deep input construction, query setup, execution, observation and disposal over changing queries. Otherwise this promising work contrast could still overstate the value of retention.

## Correctness and reproducibility

**All 48 sources pass independent complete-answer checks in both compiled feature builds, twice each.** Each run checks 576 complete backend answers: four local strategies with metrics enabled and disabled, plus four conventional access/specialization configurations. It also checks 384 one-step cancellation/preparation-reuse pairs and verifies held answers after engine and preparation disposal. The scalar results additionally satisfy exact expected hit and residual-resource counts. This does not establish cancellation at every boundary or sustained memory behavior.

**Actual graph diagnostics now follow the multihead metrics parameter.** The nested-match diagnostic test first exposed a test assumption: the plain executor's firing counter is intentionally zero. The corrected test then failed because pattern diagnostics were disabled. Passing the existing const metrics parameter through the graph enables its existing counters only in diagnostic instances. The plain instances still report zero pattern/equality counts and preserve complete answers. No matching algorithm or reference interpreter changes were needed.

The [frozen evidence](s01-structural-prefix/freeze.json) includes source bytes and both binary hashes. The [audit](s01-structural-prefix/audit.json) verifies all 384 rows per build, exact repetitions, seed agreement, zero conventional counters in the plain build, and depth contrasts. It preserves diagnostic-test failures and the development compile repair. The semantic package passes 99 tests across 18 targets; strict Clippy and seven historical evidence audits pass. Their receipts accompany the raw runs. Historical cost audits use their original frozen source bytes; the exact matching snapshot for the generic guard helper is retained with those receipts.

## Disposition and next investigation

The expensive-prefix benefit is demonstrated within this deterministic source fragment. Its total efficiency, generated/activation controls, subscriptions and broader join planning remain unanswered. Neither the favorable work counts nor the invalidation case settles the architectural family.

The [required portfolio review](S01-structural-prefix-review.md) selects useful symbolic union next. The join cost experiment remains a concrete obligation; this selection changes its order, not its evidence status. The research goal remains active.
