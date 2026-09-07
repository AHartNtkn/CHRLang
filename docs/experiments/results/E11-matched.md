# E11 first matched execution comparison

The direct arena control is cheaper than the term-expression symbolic candidate
on every completed workload in this matrix. Symbolic relational decomposition
hits the 60-second limit in all three repetitions; the direct control completes
with the four expected answers. This supports a bounded negative result for this
implementation on small workloads, not rejection of whole-evaluator compilation.

The main [36-run matrix](E11-matched-v1.jsonl) uses code `d989d79` and its frozen
registration. The [12-run carry extension](E11-matched-carry.jsonl) uses `9023371`.
Both run three fresh sequential processes per configuration, shuffled with seed
1101. Bounds are the committed E11-derived-bounds.jsonl values. Python 3.11 and
Z3 5.1.0.0 run on Linux x86_64; RSS includes runtime/backend memory.

| Workload | Direct process seconds: median [min, max] | Terms process seconds: median [min, max] | Median RSS KiB: direct / terms |
| --- | --- | --- | --- |
| Conditional occurs | .0305 [.0290, .0344] | .1620 [.1614, .1677] | 13,568 / 51,516 |
| Conditional history | .0295 [.0277, .0296] | .2943 [.2880, .3035] | 13,568 / 52,244 |
| Forward addition | .0285 [.0280, .0288] | 17.2089 [16.9958, 18.3103] | 13,568 / 63,932 |
| Relational decomposition | .0276 [.0265, .0325] | 3 timeouts at 60 s | 13,568 / unavailable |
| Type synthesis prefix | .0302 [.0285, .0346] | 1.2295 [1.1392, 1.3165] | 13,568 / 57,012 |
| Carry, one choice, zero rewrites | .0295 [.0281, .0363] | .3227 [.3017, .3365] | 13,568 / 51,852 |
| Carry, one choice, one rewrite | .0266 [.0264, .0272] | .4281 [.3983, .4333] | 13,568 / 52,320 |
| Carry, one choice, four rewrites | .0278 [.0250, .0285] | 1.6630 [1.5366, 1.6853] | 13,568 / 54,876 |

All completed paired configurations agree on raw multiplicity, answer counts,
full expected answers and both cutoff flags. Every repetition reproduces its
non-time semantic and operation counters. Type synthesis returns the expected
`k` prefix with resource-cutoff paths; it is not exhaustive synthesis. All other
completed configurations have no cutoff. Independent replay checks every
symbolic model. The direct control independently checks finite outputs and raw
multiplicity against E00; it shares no symbolic unification or matching service.

The zero-rewrite carry case does not exercise repeated common computation. The
registration's initial description overstated that aspect; the additional one-
and four-rewrite cases correct the coverage gap. Their direct source-step counts
are 12 and 24. Symbolic formula counts are not source-expansion counters, so the
matrix does not quantify source work shared by the solver.

Process time includes startup, engine work and expected-answer validation. Engine
phase measurements include backend import, construction, solving, observation,
boundary queries and, for symbolic answers, independent witness replay. Direct
answer construction/dedup happens during execution; symbolic observation happens
after construction. Phase columns therefore have different boundaries. Complete
cost is comparable; isolated phase ratios are not. Median symbolic observation
plus boundary cost for the two added carry cases is .156 and .328 seconds;
construction remains material even when this extra validation cost is visible.
The copying-metadata arena is a credible specific control, not a claim to optimal
scalar execution; persistent/trailing controls remain relevant at larger scales.

The tests pass 47 checks, including the direct arena's 121 independent unifier
comparisons and compatible transition extension. Extension is checked at T=0..8
against fresh symbolic instances and the direct control on four cases, including
resource failure and prior answer observations. Matching N/O/P/U are fixed during
extension; changing those bounds requires reconstruction.

## Disposition and next investigations

E11's first comparative package is complete; the direction remains open.
Decomposition cost attribution, measured compatible-prefix reuse, a dominating
resource-bound sequence, larger opaque continuations, stronger scalar controls,
and larger SK/lambda/type applications can still materially change the conclusion.
The current experiment does not establish fairness, useful unbounded search,
competitive scaling, or a language-design restriction. Those follow-ups stay in
the portfolio while the independent E12/E13 first probes proceed.

Reproduce the main matrix with `research/chr-symbolic/run_comparison.py MANIFEST
OUTPUT`; add `--case carry-k1-w1-n0 --case carry-k1-w4-n0` for the extension. Export
the manifest through the Rust fixture example as documented in the preceding
[E11 receipt](E11-term-encoding.md). Raw rows preserve each repetition, bound,
resource status, engine timing and available memory measurement.
