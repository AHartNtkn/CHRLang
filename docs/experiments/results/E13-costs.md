# E13: bounded entry specialization costs

The first compiler reduces dispatch but does not repay its allocation cost on any
terminating workload within three searches. This bounds the result to the current
single-entry representation and certificate; it is not a rejection of partial
evaluation, structural dispatch, or specialization for synthesis.

## Reproduction and validation

Prospective registration and harness: commit `2f3145d`,
[registration](../registrations/E13.md). Build with
`cargo build --release -p chr-specialize --example measure`, then run:

```
python research/chr-specialize/run_comparison.py docs/experiments/results/E13-costs-v1.jsonl --seed 1301
python research/chr-specialize/run_comparison.py docs/experiments/results/E13-costs-replay.jsonl --seed 1302
```

Each batch has 900 passing rows: 50 workloads, six budgets, three fresh searches
per compiled program. All non-time fields replay exactly. Workspace tests, Clippy
with warnings denied, and formatting pass. The generated arithmetic expectations
are independently checked against the reference, and all six budgets are checked
against those observations. Full residuals, raw successes, unique answers and
exhaustion are retained. S01 checks only its registered finite sibling prefix.

The zero budget invokes no compiler. Other budgets charge certification and
compilation once, plus each fresh scalar execution including answer observation.
Requested bytes measure allocation traffic, not resident memory. Live/peak byte
fields, code size, semantic work and timing are retained in the raw files.

## Results

At budgets 2, 4, 8 and 16, 32 of 50 cases perform fewer rule applications. At
budget 16, 11 cases allocate fewer runtime bytes. Only S01 has lower requested
bytes after compilation, for either one or three searches. Its fixed 200-step
prefix is not equal completed work on an infinite search and is not evidence of
whole-search amortization. No terminating workload pays back within three uses.

For forward addition at N=64:

| Expansion budget | Rule applications | Equality pairs | Compiled term nodes | One-shot requested bytes | Three-use requested bytes |
|---|---:|---:|---:|---:|---:|
| 0 | 65 | 259 | 22 | 354,662 | 1,063,986 |
| 1 | 65 | 258 | 353 | 463,050 | 1,218,322 |
| 4 | 62 | 255 | 554 | 732,007 | 1,506,281 |
| 16 | 50 | 243 | 1,268 | 3,130,477 | 3,973,791 |

One-shot times for budget zero are 611/630 microseconds across the two batches;
budget 16 takes 7,467/6,110. Three-use totals are 1,588/1,608 versus 8,317/6,997.
These are two observations, not a stable hardware speed estimate.

At N=64 decomposition, budget 16 reduces applications 65 to 50 but leaves equality
pairs at 324. One-shot requested bytes increase 929,002 to 3,888,846. Repeated-variable
addition reduces applications 129 to 114 and equality pairs 4,740 to 4,605, while
one-shot requested bytes increase 917,232 to 7,447,271. This separates dispatch
compression from the cost of constructing and instantiating specialized syntax.

## Interpretation and feasible follow-ups

The current compiler repeats known constructor structure in private entry heads,
retains original definitions for residual calls, and builds owned syntax while
unfolding. These are concrete possible explanations, not experimentally isolated
causes. Register a paired static-argument-elimination ablation to distinguish entry
matching/storage from remaining body and compilation costs. Measure shared syntax
or staged substitution separately if the first ablation leaves those costs dominant.

Broader source-boundary handling and structural dispatch remain necessary to study
SK/type/lambda synthesis, which this first certificate excludes. Multiple variants,
code growth policies and longer reuse are separate feasible questions. None is
closed by these results. The first E13 package is complete; E06 and E09 can proceed
independently while these refinements remain on the coverage map. No production
architecture or language restriction is adopted.
