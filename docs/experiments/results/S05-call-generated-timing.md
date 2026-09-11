# Call reuse retains gains after generated matching

**Four repeated-query cases qualify faster with traces than with every tested
control, including both generated configurations.** Generated matching also has
fifteen qualified gains against generic indexed execution. These results preserve
a real time–memory tradeoff; they do not select generation or reuse universally.

## Complete runtime costs

Each row contains 144 scenarios. Ratios divide candidate time by control time;
lower is better. “Uncertain” means the registered ten-repetition rule did not
establish both direction and a 10% effect. It does not mean equivalence.

| Candidate / control | Faster / slower / uncertain | Median ratio range |
|---|---:|---:|
| Trace / Direct | 12 / 18 / 114 | 0.604–1.849 |
| Trace / compiled scan | 17 / 12 / 115 | 0.571–1.635 |
| Trace / generic indexed | 35 / 0 / 109 | 0.454–1.331 |
| Trace / inferred | 16 / 6 / 122 | 0.504–1.665 |
| Trace / generated | 18 / 7 / 119 | 0.534–1.461 |
| Trace / generated plus inferred | 21 / 18 / 105 | 0.568–1.900 |
| Generated / generic indexed | 15 / 0 / 129 | 0.585–1.153 |
| Generated plus inferred / generic inferred | 0 / 0 / 144 | 0.753–1.367 |

All 10,080 processes passed, with no excluded samples. The 1,152 comparisons
exceed the 2,300 ns clock floor. The sum includes source construction,
preparation, query input/setup, execution and observation, consumption and every
disposal phase. Complete answers are validated outside intervals. All binaries
are frozen, counter-free and use the ordinary allocator; no builds ran alongside
timing. Process startup and compiler costs are not included in these sums.

## Where the trace gains survive

The four all-control gains use depth 128 and four repeated queries:

| Family | Consumer behavior | Trace/control median range across six controls |
|---|---|---:|
| Ordinary success | Keep all, exhaust | 0.531–0.757 |
| Failed alternative | Keep all, cancel after first answer | 0.504–0.743 |
| Binding before private work finishes | Drop outputs, cancel after first answer | 0.526–0.787 |
| Binding before private work finishes | Keep all, cancel after first answer | 0.495–0.841 |

Three of these cases were among the preceding eight all-generic-control gains.
The remaining prior cases are not discarded or treated as disproven; their exact
current classifications appear in the [diagnosis](s05-call-generated-timing/diagnosis.json).
The changing membership reinforces the need to retain uncertainty at case level.

Against generated matching, seventeen of eighteen trace gains use repeated
queries. Against generated plus inference, eighteen of twenty-one do. Neither
comparison has a qualified trace gain for four distinct queries. Retaining reusable
work is still most promising when it is actually revisited.

Heap costs remain part of this choice. Across all scenarios, trace/generated
requested-byte ratios are 0.602–1.588 and peak ratios 0.690–3.008. Against generated
plus inference they are 0.630–1.681 and 0.669–3.630. These are the independently
qualified exact heap measurements, not allocation instrumentation inside timing.
Requested heap bytes are not RSS.

## Generated execution needs stronger comparisons

All fifteen generated/indexed gains use four queries: thirteen at depth 128 and
two at depth 32, split between repeated and distinct queries. Their median service
phase savings are about 177–597 microseconds per session. This identifies matching
as a consequential source of the runtime benefit, but not its full lifecycle value.

A post-measurement screen against **all four generic controls** changes the next
investigation. Ordinary generated matching has no scenario with a median at least
10% below every generic control. Generated plus inference has fourteen such
scenarios, yet its registered comparison with generic inference remains uncertain
in all 144 cases. Those fourteen are hypotheses for a follow-up, not additional
qualified wins selected after seeing results.

The uncertainty is not explained by one uniformly adverse block: across all ten
blocks the median generated-inferred/inferred ratio ranges from 0.920 to 0.973,
with both directions present within every block. Individual comparisons vary
enough to fail the registered rule. Preserve all blocks and test longer sessions;
do not silently promote favorable medians or remove inconvenient repetitions.

## Next experiment

**Keep T075 active for longer prepared sessions and compiler amortization, with
Direct, scan and inferred execution included.** An indexed-only compiler contrast
would leave the stronger-control question unanswered. Use the existing emitter
and compilation workflow; extend the shared lifecycle runner's query accounting
rather than impose its current four-query fixture size on the experiment.

Register user-program emission, replicated compilation, artifact ownership and
complete runtime before runs. Separate shared dependency construction from the
user-program increment. Select long-query cases spanning the fourteen promising
inferred-generated scenarios, ordinary generated gains, and adverse controls;
freeze those choices before measurement. Validate every delivered answer and
cancellation prefix outside measured intervals, including changed queries and
final ownership. An analytical crossover estimate may size runs; it is not an
observed amortization result.

This next comparison can determine whether generated matching provides an
architectural benefit beyond a weaker indexed control, and whether that benefit
repays compilation. Bounded trace retention, broader observers and sparse
projection remain required alternatives. This is package one after the last full
portfolio review; the research goal remains active.

[Registration](../registrations/S05-call-generated-timing.md) ·
[Receipts and frozen inputs](s05-call-generated-timing/) ·
[Complete analysis](s05-call-generated-timing/analysis.json) ·
[Source and ownership qualification](S05-call-generated-entry.md).

Reproduce with `call_generated_timing.py audit` and
`call_generated_timing_diagnosis.py audit` in `research/chr-reuse/experiments/`.
