# Contextual lifecycle sizing exposes a memory tradeoff

**The contextual path uses less peak requested memory than the controls on the branching cases, while allocating more total bytes than scanning.** Single timing samples suggest a benefit over relational execution but do not establish a timing advantage over the compiled controls. All runs completed with correct answers; confirmation still needs a stronger direct-lowering control.

## What was varied

The four modes are contextual ownership, corrected relational ownership, compiled global scanning and compiled global indexing. The sources independently vary no choice versus three binary choices, no branch-local bindings versus eight local bindings, and no consumption versus eight distinct keyed token claims. Payload depth is 0, 8 or 32 independently of those choices. Eight hold occurrences remain in every query. Preparation serves one or four queries, alternating payload depth and insertion order.

This produces 192 configurations. Each has one ordinary-allocator timing sample, in prospectively shuffled order. Depth32/four-query configurations also have two allocation replays each, totaling 64 diagnostic processes. Eight more processes check cancellation after one tick followed by a complete query over the same preparation. All complete answers are compared with the independent scalar evaluator outside measured phases. Allocation replays agree exactly phase by phase, and every query/prepared disposal restores the requested-allocation baseline.

The [registration](../registrations/S02-contextual-lifecycle-sizing.md) fixes sources, sizes, order and bounds. The [runner](../../../research/chr-relational/examples/s02_contextual.rs) reuses the established lifecycle measurement structure, while the [source generator](../../../research/chr-relational/examples/support/contextual_source.rs) supplies this experiment's independent dimensions. The process launcher applies CPU affinity, 60-second/1-GiB limits and a two-million-advance bound. No cutoff occurred.

## Exploratory timings

The following milliseconds sum preparation, setup, execution with complete observation, engine/answer disposal and prepared disposal. They are single samples for four changed queries at depth32 with consumption. They must not be read as confirmed rankings.

| Source | Contextual | Relational | Scan | Indexed |
|---|---:|---:|---:|---:|
| No choice, unchanged local variables | 0.151 | 0.349 | 0.205 | 0.265 |
| No choice, eight local bindings | 0.237 | 0.498 | 0.267 | 0.312 |
| Three choices, unchanged local variables | 0.746 | 1.363 | 0.800 | 0.941 |
| Three choices, eight local bindings | 1.323 | 2.076 | 1.299 | 1.330 |

Source/input construction and first-answer latency are separately recorded. Execution and observation share an interval; fixtures, validation and reporting remain outside those intervals and inside host wall time. Compilation is not isolated. Primary timing builds disable compiled engine/kernel/observer counters and use the ordinary allocator; allocation runs are separate.

## Repeated allocation evidence

For the branching consuming cases at depth32/four queries, the allocation meter reports:

| Source and mode | Cumulative requested bytes | Peak requested growth above host baseline |
|---|---:|---:|
| Unchanged locals, contextual | 1,437,431 | 50,037 |
| Unchanged locals, relational | 2,636,418 | 493,788 |
| Unchanged locals, scan | 931,353 | 112,412 |
| Eight local bindings, contextual | 2,956,087 | 54,741 |
| Eight local bindings, relational | 3,803,438 | 512,253 |
| Eight local bindings, scan | 1,277,305 | 125,788 |

The contextual path's lower peak is consistent with shared constructor storage and context-local maps. This is not a causal allocation decomposition: the paths also differ in discovery and representation. Its higher traffic than scanning shows that a smaller live footprint does not imply less allocation work. These are requested heap bytes, not RSS or sustained-memory bounds; host fixtures are excluded from the growth baseline.

## Consequence and next action

The measurements justify a confirmatory comparison, not architecture selection. The timing boundary against scanning is close enough that repeated randomized measurements matter. The lower contextual peak is useful contrary evidence to consider alongside allocation traffic and runtime, rather than collapsing them into a weighted score.

These sources are regular enough to invite direct lowering: their choices, fixed token claims, bindings and output construction have a simple source-specific derivation. A four-executor comparison would be incomplete if that direct path avoids most of their machinery. Add an exact eligibility check and independently validate the lowering before freezing confirmation. Charge checking, preparation, construction, complete output and disposal; do not claim a general compiler from one schema.

The [four-package breadth review](S02-first-breadth-review.md) compares this next step with still-unimplemented alternatives. Broader integrated mechanisms, source lowering and lifetime remain open. No universal winner or runtime portfolio is selected.

[Raw analysis](s02-contextual-sizing/analysis.json), [source/binary freeze](s02-contextual-sizing/freeze.json), [verification](s02-contextual-sizing/verification.json), [source/cancellation gate](s02-contextual-sizing/source-gate.log), [meter self-check](s02-contextual-sizing/meter-check.log), and [Clippy](s02-contextual-sizing/clippy.log). Raw per-process records and build logs are adjacent. Strict Clippy passes for the example; the integrated dependency emits its existing build-script dead-code warnings.
