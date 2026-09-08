# Direct finite-table compilation repays preparation in this pilot

Direct source-derived joins beat both generic dedicated controls in every registered family, including one-query sessions. The advantage survives preparation, observation and disposal costs; it is not dependent on assuming long preparation reuse. This supports compiling the admitted finite table fragment directly, while leaving broader compilation and language adoption open.

All 252 registered processes completed with correct answers and no cutoffs. The 36 allocation cells replay exactly. Results apply to the sealed source certificate established by the [source gate](S06-table-source-gate.md), not arbitrary CHR, native code generation or every relation-solving algorithm.

## Complete session costs

The table shows median lifecycle milliseconds. A one-query session uses free arguments except in the contradictory family. A 16-query session cycles free arguments, a given first value, repeated first/third arguments and a given middle value. Each session includes fresh source preparation and final disposal.

| Family | Direct, one query | Scanned, one query | Direct, 16 queries | Scanned, 16 queries |
|---|---:|---:|---:|---:|
| Sparse, 32-value diagonal tables | 0.0633 | 4.9928 | 0.3345 | 84.9960 |
| Dense, four-value binary tables | 0.0490 | 0.5840 | 0.3939 | 9.3622 |
| Duplicate rows | 0.0137 | 0.1089 | 0.0873 | 1.6393 |
| Disconnected factors | 0.0171 | 0.0897 | 0.0971 | 1.2232 |
| Contradictory first value | 0.0072 | 0.0229 | 0.0244 | 0.2209 |
| Tiny, one-value relation | 0.0053 | 0.0148 | 0.0400 | 0.1298 |

All direct/control paired medians clear the registered 20% practical threshold, with all five repetitions favoring direct execution. Against scanning, paired direct/control ratios range from 0.0128 to 0.3593 for one query and 0.0041 to 0.3258 for 16 queries. Indexed controls also lose every registered comparison. [Full cell results](s06-table-lifecycle/summary.csv) and [paired ratios](s06-table-lifecycle/summary.json) preserve their costs and ranges.

These sources are distinct mechanism probes, not workload weights. The sparse source has more table rows than the dense source; the table does not compare those families as equivalent amounts of work. Single and reused sessions also use different query mixtures, so their medians must not be fitted as a universal linear amortization curve.

## Why direct execution helps

The direct path selects compatible table rows through shared argument positions and column indexes. Source execution instead creates choice alternatives, performs their equalities, manages successful and failed branches and discovers subsequent source applications. The accepted source has a proof of finite relational correspondence, so this intermediate execution is avoidable on that fragment.

The largest contrast is the sparse table source, where most combinations of separately chosen rows disagree. Direct execution requests about 0.623 MiB over 16 queries; scanning requests about 129.950 MiB for the same 264 complete answers. This is requested allocation traffic, not RSS. Duplicate and disconnected families confirm that the gain does not depend on suppressing raw answer multiplicity.

The direct compiler's source inspection and index construction also cost less than generic preparation here. In the sparse case, median direct preparation is about 0.029 ms versus 0.040 ms for scanning. In the tiny case it is about 0.0022 ms versus 0.0047 ms for a one-query session. Thus this pilot does not expose a preparation crossover against these generic controls. It does not prove that index construction is cheaper for larger tables, other certificates or generated execution.

Contradictory queries all exhaust with zero answers; first-answer latency is null rather than fabricated. Output-heavy queries retain all answers until measured disposal. The runner includes joint execution/observation because both direct enumeration and source-branch observation produce answers incrementally. Branch disposal internal to that loop remains charged there. First-answer latency overlaps that phase and is not added to total time.

## Architectural and language consequences

The supported choice is direct relation compilation for the tested admitted programs, rather than interpreting their table-choice rules. It removes occurrence storage, activation, propagation history and source branch execution from that path. It still requires a certificate, extracted tables, indexes, substitutions, continuations and exact observation.

The whole language need not adopt this fragment's restrictions. Sealed rulesets, ground finite rows and absence of observers currently justify changing execution order. Arbitrary safe guards, partial rows, contextual effects, recursion and unsupported source regions require further certificates or a different compiler. A mandatory restricted language and an optional optimization with boundaries have different complexity and expressiveness costs; this pilot does not choose between them.

The results do not rank R04's trailed finite solver, a generalized constraint solver, another multiway join algorithm or native generation. Their representations and preparation obligations differ. The ordinary controls use generic source execution, and the comparison cannot establish supremacy over all compiled organizations. General table size/reuse crossovers and ongoing output consumers remain open, despite the consistent measured advantage here.

## Evidence and next selection

The [prospective registration](../registrations/S06-table-lifecycle.md), [source/binary freeze](s06-table-lifecycle/freeze.json), [raw 252 processes](s06-table-lifecycle/raw.jsonl), [runner](../../../research/chr-direct-relation/examples/s06_lifecycle.rs) and [analysis](../scripts/s06_summarize.py) record exact inputs and accounting. Before the matrix, all 72 engine configurations and 24 scalar-oracle configurations passed. Tests, strict Clippy and formatting pass. Timing uses counter-free release builds with the ordinary allocator; allocation runs use a separate meter. Native compilation, startup, input syntax construction and validation are excluded.

T064's bounded finite-table trial is complete. S06 remains open for broader relation/structural solvers, recursive/contextual lowering, native compilation and held-out challenges. The next task returns to [S01 selective and consuming discovery](S01-next-selection.md), whose maintained-join alternatives have correctness foundations but still lack the intended discriminating source and lifecycle comparison. Another finite-table tuning pass is less valuable now than resolving that ordinary-computation contrast.
