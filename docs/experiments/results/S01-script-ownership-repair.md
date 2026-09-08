# Correct script ownership changes the control comparison

Owning one source script instead of every remaining suffix removes most of the long-script overhead. Both lowerings now beat the generic controls in every registered family. Retaining pairs still has a consequential cost on dense consumption, while its possible benefit for repeated selective requests needs a focused crossover check.

All 336 paired processes completed and validated. The corrected matrix has 32 cells; 16 original Direct/Retained cells were rerun in the same randomized blocks. All 48 allocation diagnostics replay exactly. The measured lifecycle harness and fixtures were byte-identical between original and corrected binaries.

## What changed and why it is correct

The decoder now borrows one owned script and records each instruction's position. It clones only the operation arguments needed by execution. If replacement blocks, it finds that position in the original script and reconstructs the exact remaining driver term. This changes ownership cost without weakening blocked-query observation or changing rule semantics.

The source gate checks a blocked replacement after earlier binding and consumption, including its later unexecuted instructions. Default and counter-free gates pass, as do Clippy, formatting and the full compiled-package suite. The normal input still owns its source script until engine disposal, and that disposal remains measured.

## Paired correction results

Ratios divide corrected lifecycle by freshly measured original lifecycle within each repetition; lower values favor the correction.

| Family | Direct ratio | Retained ratio |
|---|---:|---:|
| Selective consumption | 0.180 | 0.189 |
| Dense consumption | 0.182 | 0.233 |
| Broad replacement | 0.146 | 0.151 |
| Sparse replacement | 0.622 | 0.622 |
| Sparse binding | 0.815 | 0.762 |
| Broad binding | 0.916 | 0.923 |
| Stable selective requests | 0.866 | 0.841 |
| Stable dense requests | 1.006 | 0.945 |

Consumption and replacement improvements pass the registered practical threshold and same-direction condition. Sparse binding passes that threshold for Retained but not Direct. The remaining cells do not establish a consequential change at this precision; some contain timing outliers. There is no practically consequential regression under the registered criterion.

Requested traffic also falls substantially. Selective consumption changes from 2.991 to 0.437 MiB for Direct and 3.051 to 0.497 MiB for Retained. Broad replacement changes from 4.949 to 0.559 MiB for Direct and 5.014 to 0.624 MiB for Retained. These are requested heap bytes, not RSS or isolated engine peaks.

## What the corrected comparison says about retention

Dense consumption is now a clear adverse case for retained pairs. The paired Retained/Direct lifecycle ratio is 1.401, range 1.319–1.463. Many retained pairs are constructed and invalidated for relatively few consuming receipts. At N=8, the within-query ratios are about 1.55 for both one and eight requests. This is a cost of this maintained relation, not a correctness defect.

No other full-session Retained/Direct contrast reaches the 20% practical threshold. Broad binding favors Retained by about 10%, sparse binding by about 16%, and broad replacement favors Direct by about 5%. Selective consumption and sparse replacement have little measured difference. Stable selective requests remain noisy.

Per-query results make further investigation worthwhile rather than closing the question. For stable selective N=8/R=8, the paired median ratio is 0.813 but the range crosses one. Sparse binding at eight requests lies near the practical threshold. Larger request frequency could repay setup and maintenance, while larger dense consumption could amplify the adverse regime. The current mixture of queries is a registered session, not a workload weighting or a universal amortization curve.

Both corrected lowerings beat generic controls across the registered complete sessions. The strongest lesson here is that compiled source execution and appropriate ownership matter; this does not select Retained over Direct, nor establish superiority over general generated access or other retained-join representations.

## Evidence and next action

The [paired registration](../registrations/S01-script-ownership-repair.md), [freeze](s01-script-repair/freeze.json), [raw processes](s01-script-repair/raw.jsonl), [correction ratios](s01-script-repair/correction.json), [query ratios](s01-script-repair/query-ratios.json) and [full cell summaries](s01-script-repair/summary.csv) support the result. [Final validation](s01-script-repair/validation.json) records package tests and the corrected runner gate.

A package-wide check required moving the example's counter-free assertion from compile-time evaluation to runtime, so default-feature test builds can compile the example. The assertion still runs before any measurement. The [measured runner](s01-script-repair/measured-runner.rs) and [precise validation-only change](s01-script-repair/runner-validation-change.json) preserve the measured source; no measured interval or fixture changed.

T065 remains active for a focused request-frequency and row-count crossover investigation, with stable selective requests and dense consumption as favorable/adverse anchors. Register that follow-up before runs and avoid repeating the full generic matrix unnecessarily. S04 restoration/replay remains the strongest independent alternative; this follow-up is justified by a concrete, near-threshold decision uncertainty and must not become an indefinite series of local tuning passes. S01's broader partial-join, subscription and generated-access mechanisms remain open.
