# Coarser recognition lowers key ownership, with a measurable missed-reuse cost

**Both wider recognition strides reduce requested and peak heap in every scenario versus stride one. Both still exceed Direct, nonmemoized separation and sealed compiled execution in every scenario.** Stride sixteen also loses to stride four in 72 requested-allocation comparisons: recognizing less often is not a monotonic improvement.

The [fresh registration](../registrations/S05-recognition-stride-exact-ownership.md) repeats all factors from the [initial registration](../registrations/S05-recognition-stride-ownership.md). The accepted campaign validates 4,096 processes, 2,048 exact repetition pairs and 256 scenarios with equal consumer-owned bytes across all eight modes. Complete answers pass the independent evaluator; every session restores its initial live heap after final disposal. The meter counts requested heap, not RSS or ordinary execution time.

## A real output-ownership defect was repaired

The initial 4,096 processes completed but failed the consumer-ownership check in 448 cells. Separated answer assembly appended detached residuals using geometric vector growth, retaining unused capacity in delivered answers. A regression test reproduced that defect. The shared assembly path now reserves the known additional length exactly before extending the vector. The allocation/reallocation cost remains inside measured service/observation.

The initial campaign and its [qualification failure](s05-recognition-stride-ownership/qualification-failure.json) remain recorded. Its comparisons are not accepted. The fresh campaign uses the repaired source and satisfies the original criteria without exclusions. Metrics-on/off semantic tests and scoped Clippy pass.

## Complete ownership comparisons

Each row compares all 256 scenarios. Ratios are candidate/reference; ranges are unweighted.

| Candidate / reference | Requested ratio range | Peak-live ratio range | Result |
|---|---:|---:|---|
| Stride 4 / stride 1 | 0.402–0.704 | 0.315–0.649 | Lower in all 256 |
| Stride 16 / stride 1 | 0.251–0.810 | 0.151–0.609 | Lower in all 256 |
| Stride 4 / Direct | 1.375–3.725 | 1.480–8.582 | Higher in all 256 |
| Stride 16 / Direct | 1.137–2.326 | 1.263–4.122 | Higher in all 256 |
| Stride 4 / nonmemoized separation | 1.207–2.994 | 1.484–8.398 | Higher in all 256 |
| Stride 16 / nonmemoized separation | 1.005–1.951 | 1.267–4.033 | Higher in all 256 |
| Stride 4 / sealed compiled | 1.130–6.104 | 1.114–5.899 | Higher in all 256 |
| Stride 16 / sealed compiled | 1.149–3.930 | 1.084–2.857 | Higher in all 256 |

Against generic indexed execution, stride four requests less in 44 cases and peaks lower in 64; stride sixteen requests less in 40 and peaks lower in 56. The stronger sealed and Direct controls prevent interpreting these selective indexed advantages as an architecture choice. Scanning receipts are also retained.

The lifecycle includes source construction, preparation, one/four alpha-renamed query uses, input/setup, service/observation, immediate or retained consumption, cancellation after the first answer, and all disposal. Families include readable and unreadable observations, variable-dependent binding, arity near misses, duplicate growing residuals, failure, a no-reuse branch and consuming completion with propagation history. Memo tables remain query-local; this is not cross-query result caching.

## Why stride sixteen can cost more than four

Stride sixteen requests more in 72 cases and peaks higher in 64. In the shallow equal-tag source with four prepared uses, immediate output disposal and cancellation, it requests 168,512 bytes versus stride four's 127,804; peak live bytes are 18,090 versus 16,203.

A [source witness](../../../research/chr-reuse/tests/recognition_cost_witness.rs) reproduces the mechanism at depth four:

| Stride | Key requests | Executed transitions | Reused transitions, complete/cancel | Retained states |
|---|---:|---:|---:|---:|
| 1 | 22 | 21 | 18 / 17 | 21 |
| 4 | 7 | 22 | 17 / 16 | 22 |
| 16 | 3 | 34 | 5 / 4 | 34 |

Fewer recognition points miss an earlier convergence. The wider stride recomputes and retains more of the derivation. This is direct work/state evidence for a tradeoff, not an attribution of every allocated byte to one function.

## Next investigation

Keep T075 active for ordinary lifecycle timing of the repaired modes against the same controls. The key reduction is substantial, but lower allocation versus stride one does not show whether either candidate repays reuse costs against Direct. Register fresh counter-free timing only after exact ownership entries pass.

Call-boundary recognition and direct solving remain alternatives to this periodic policy. A whole-call cache needs a justified scheduling boundary; direct solving needs source eligibility and resource semantics. Measuring the current qualified mechanism is now a cheaper discriminating step than implementing either alternative immediately. Reassess them after timing. This is package two after the full portfolio review; broader recognition, failure, relevance, eviction and sustained-ownership questions remain required.

Reproduce with `python3 research/chr-reuse/experiments/recognition_ownership.py --exact-output --audit`. [Accepted summaries](s05-recognition-stride-exact-ownership/analysis.json), full phase receipts and frozen sources are retained. Run the diagnosis with `cargo test -p chr-reuse --test recognition_cost_witness -- --nocapture`. The research goal remains active.
