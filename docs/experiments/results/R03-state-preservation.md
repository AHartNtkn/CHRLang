# State preservation: copying warrants owner attribution

Preserving live state across cheap failing alternatives is consequential in this screen. At 512 payloads and 64 alternatives, split-producing service requests about 84% of measured heap traffic. Primary complete lifecycle is 23.733 ms, versus 7.852 ms with no choice. The result warrants identifying copied owners before choosing a snapshot intervention; it does not predict a cloning speedup.

The [prospective registration](../registrations/R03-state-preservation.md) was committed at `28b71ce` before runs. All 288 processes and all 36 cells complete, validating 9,120 full answers and exact failures/exhaustion. All 36 allocation processes restore their baselines. The exact seeded manifest and source/binary hashes pass both the analyzer and independent audit. Maximum process wall time is 0.926 seconds and maximum service ticks per query 266,828, within the 30-second/20-million bounds. [Metadata](r03-state-preservation/metadata.json), [raw outcomes](r03-state-preservation/runs.jsonl), [audit](r03-state-preservation/audit.json) and [all diagnostic summaries](r03-state-preservation/summary.json) are retained.

## Complete lifecycle

Milliseconds per query, median [minimum,maximum] across five ordinary-allocator primary processes. Preparation and prepared disposal are amortized across the query count. Four-query cells alternate n and n+1, so q4/n0 is not a wholly zero-state control. These are complete measured backend lifecycles; compiler/startup costs are not isolated.

| Payloads | Alternatives | Outcome | Queries | Lifecycle ms |
|---|---|---|---|---|
| 0 | 1 | mostly-fail | 1 | 0.060 [0.052, 0.073] |
| 0 | 1 | mostly-fail | 4 | 0.026 [0.023, 0.027] |
| 0 | 1 | all-success | 1 | 0.061 [0.051, 0.087] |
| 0 | 1 | all-success | 4 | 0.027 [0.022, 0.059] |
| 0 | 8 | mostly-fail | 1 | 0.083 [0.081, 0.102] |
| 0 | 8 | mostly-fail | 4 | 0.052 [0.049, 0.058] |
| 0 | 8 | all-success | 1 | 0.127 [0.109, 0.142] |
| 0 | 8 | all-success | 4 | 0.101 [0.087, 0.228] |
| 0 | 64 | mostly-fail | 1 | 0.670 [0.590, 1.141] |
| 0 | 64 | mostly-fail | 4 | 0.654 [0.621, 0.763] |
| 0 | 64 | all-success | 1 | 0.953 [0.882, 0.992] |
| 0 | 64 | all-success | 4 | 1.054 [1.031, 1.084] |
| 64 | 1 | mostly-fail | 1 | 0.596 [0.552, 0.624] |
| 64 | 1 | mostly-fail | 4 | 0.549 [0.526, 0.575] |
| 64 | 1 | all-success | 1 | 0.599 [0.557, 0.620] |
| 64 | 1 | all-success | 4 | 0.518 [0.502, 0.531] |
| 64 | 8 | mostly-fail | 1 | 0.781 [0.755, 0.939] |
| 64 | 8 | mostly-fail | 4 | 0.780 [0.718, 0.826] |
| 64 | 8 | all-success | 1 | 1.981 [1.855, 2.110] |
| 64 | 8 | all-success | 4 | 1.979 [1.892, 2.009] |
| 64 | 64 | mostly-fail | 1 | 2.723 [2.577, 3.087] |
| 64 | 64 | mostly-fail | 4 | 2.851 [2.807, 2.894] |
| 64 | 64 | all-success | 1 | 16.858 [16.139, 18.632] |
| 64 | 64 | all-success | 4 | 15.327 [14.901, 17.715] |
| 512 | 1 | mostly-fail | 1 | 7.852 [7.566, 8.016] |
| 512 | 1 | mostly-fail | 4 | 7.940 [7.893, 7.997] |
| 512 | 1 | all-success | 1 | 7.764 [7.481, 10.764] |
| 512 | 1 | all-success | 4 | 8.000 [7.849, 8.367] |
| 512 | 8 | mostly-fail | 1 | 9.794 [9.478, 9.845] |
| 512 | 8 | mostly-fail | 4 | 9.777 [9.622, 9.832] |
| 512 | 8 | all-success | 1 | 22.006 [21.284, 22.108] |
| 512 | 8 | all-success | 4 | 21.175 [20.927, 21.390] |
| 512 | 64 | mostly-fail | 1 | 23.733 [23.335, 24.160] |
| 512 | 64 | mostly-fail | 4 | 23.879 [23.708, 26.062] |
| 512 | 64 | all-success | 1 | 181.714 [179.922, 190.918] |
| 512 | 64 | all-success | 4 | 174.019 [171.935, 175.277] |

## What the diagnostics establish

All four largest-state mostly-failing cells meet the registered owner-inspection trigger. With 8 alternatives, split-producing service accounts for 36.7% of requested traffic and 14.7–14.9% of diagnostic lifecycle. With 64 alternatives it accounts for 83.8% of traffic and 46.1–46.6% of diagnostic lifecycle. At fixed alternatives, requested traffic per split grows from 3,754 bytes (n0/a8/cold) or 14,561 bytes (n0/a64/cold) to about650,000 bytes at n512. Varying alternatives alone also changes the inherited alternative spine, which the zero-state controls expose.

These timings come from separate allocation-instrumented processes and include source stepping, frontier and lineage handling. They are neither isolated clone time nor a primary speedup bound. For n512/a64/mostly-fail/cold, diagnostics record40,956,306 requested bytes in 63 split events, out of 48,882,617 measured lifecycle bytes. Terminal branch disposal adds 8.930 diagnostic milliseconds inside service. The tiny final search-engine-drop interval therefore does not imply cheap lifetime costs.

The contrary controls are material. No-choice n512 already costs 7.852 ms. All-success n512/a64 costs 181.714 ms cold, with split service only 11.2% of diagnostic lifecycle. Its 33,409 source applications include cleanup for 64 successful branches, versus 1,090 applications in the mostly-failing case. Source service and disposal are more consequential there; optimizing forks would not remove required cleanup and publication. Peak requested heap is 1,427,036 bytes in mostly-failing n512/a64/cold and 41,075,680 in all-success. These are requested live peaks, not RSS.

Preparation and small full observations are minor in the largest cases: cold mostly-failing n512/a64 preparation median is 0.027 ms, while its query construction/setup medians are 0.052/0.216 ms. Full observation still preserves duplicate residuals and joint aliases. First service observation is 23.314 ms; all-success first observation is 161.402 ms under the current round-robin policy. No alternate scheduling comparison follows.

## Source ownership and the next causal gate

Read-only inspection distinguishes owners that are already shared from those copied at a fork. Bindings use a persistent Rc tree. Prepared rules and plans use Arc. The arena, ordinary occurrence/index/dependency containers, queues and pending work have copying costs. Arena cloning copies node data and interning keys; split aggregate evidence does not assign bytes to those owners individually.

One plausible intervention is arena sharing with copy-on-write only after an interning miss, but it is not selected for implementation yet. Lookup before detachment matters: eager detachment could simply move copying to the next source step. Whole-state copy-on-write has an additional concern because each continuation soon mutates occurrence/index state. A diagnostic owner breakdown and post-fork interning hit/miss counts can distinguish these cases without inventing a broad trail engine.

The same screen exposes an independently attributable construction cost. Each fresh binding to `cell(s^k(z),H,H)` visits the cell, the k+1 closed key nodes and two H references: k+4 occurs visits. Summing k=1..512 gives 133,376, exactly the work count in both no-choice and 64-alternative mostly-failing runs. Current occurs traversal does not consult existing immutable closed-subtree metadata. This explains common setup/source construction work, not the fork traffic. A sound shortcut still needs executable open-variable, indirect-cycle and branch tests; counts alone do not establish its lifecycle benefit.

T050 is complete. T051 selects a bounded causal package: validate the closed-subtree occurs shortcut, and attribute actual fork allocations by owner plus subsequent interning behavior before choosing one snapshot alternative. Source semantics and ordinary scheduling remain the responsibility boundary. The shortcut has a precise existing invariant and low implementation burden; owner attribution addresses the newly measured general search cost. Both have greater current value than the carrier-contraction gate, which could qualify a pure-countdown sharing case but needs a new scheduling/certificate argument. The latter remains a concrete alternative, not a rejected possibility.

The explicit baseline recommendation survives, with its snapshot choice still open. No universal persistence/trailing/copying ranking, mandatory language restriction or goal closure follows. Any comparative correction or snapshot measurement needs its own prospective registration and full controls.
