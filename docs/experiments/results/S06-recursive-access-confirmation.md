# Scan changes the case for recursive contraction

**A competent Scan control removes much of the earlier Indexed advantage for contraction.** Some contraction gains survive, but most Scan comparisons remain unresolved. The result supports conditional use, not a general need to replace ordinary source execution.

The [registered comparison](../registrations/S06-recursive-access-confirmation.md) completes 8,200 processes over 80 source configurations and ten build/access configurations. All 800 allocation pairs replay exactly, full independent answers agree, source-work signatures match, and query/prepared requested-live bytes are restored.

## The stronger control changes the conclusion

Contraction has 32 practical gains against same-build Indexed specialization, but only eight against Scan specialization. Scan itself gains in 44 configurations against feature-off Indexed specialization, with no classified losses. This qualifies the earlier exploratory result: avoiding an expensive access policy is a competing explanation for much of the apparent benefit.

Against feature-off Scan specialization, contracted Scan has eight gains, one loss and 71 unresolved cases. This retains a real favorable regime while withholding a general preference. The two feature-on/off specialization comparisons are unresolved in all 80 configurations; that is not proof of zero feature overhead.

## Registered practical comparisons

A gain means the upper pointwise 95% bootstrap bound on the paired primary time ratio is below 0.90; a loss means the lower bound exceeds 1.10. Seven paired blocks and 10,000 resamples follow the prospective seeds. No outliers were excluded. Counts describe the selected cases, without simultaneous/population claims or workload weights.

| Numerator / denominator | Gains | Losses | Unresolved |
|---|---:|---:|---:|
| on/contracted-scan / on/sealed-scan | 8 | 2 | 70 |
| on/contracted / on/sealed | 32 | 1 | 47 |
| off/sealed-scan / off/sealed | 44 | 0 | 36 |
| on/contracted-scan / on/contracted | 29 | 0 | 51 |
| on/contracted-scan / off/sealed-scan | 8 | 1 | 71 |
| on/sealed / off/sealed | 0 | 0 | 80 |
| on/sealed-scan / off/sealed-scan | 0 | 0 | 80 |
| off/original-scan / off/original | 50 | 0 | 30 |
| on/contracted-scan / off/original-scan | 35 | 1 | 44 |

Names without "-scan" use Indexed access. "sealed" means inferred specialization; "original" retains ordinary selection. All modes use Global source order. The full [summary](s06-recursive-access-confirmation/summary.json) records every interval, phase and work count.

Representative median primary milliseconds at depth64/four queries/resources:

| Family | Off specialized, Indexed | Off specialized, Scan | Contracted, Indexed | Contracted, Scan |
|---|---:|---:|---:|---:|
| pass | 0.303 | 0.216 | 0.171 | 0.157 |
| unary | 1.070 | 0.345 | 0.383 | 0.302 |
| nested | 1.815 | 0.479 | 0.540 | 0.437 |
| open | 1.400 | 0.292 | 0.294 | 0.232 |
| late | 1.596 | 0.351 | 0.483 | 0.306 |
| malformed | 0.897 | 0.289 | 0.275 | 0.219 |
| choice | 1.186 | 0.470 | 0.558 | 0.411 |
| multi | 1.871 | 0.559 | 1.958 | 0.540 |
| multi-choice | 2.453 | 1.115 | 2.625 | 1.100 |
| fail | 0.366 | 0.264 | 0.221 | 0.223 |

## Work evidence identifies the maintenance cost

For the unary example, same-build Indexed specialization makes 9,498 dependency visits and 540 refreshes/index repairs. Scan makes zero of each while retaining the same 266 source-equivalent applications, 540 counted candidates and 588 service steps. The nested example similarly changes from 26,914 dependency visits to zero.

Contracted Scan retains a distinct reduction: 32 candidates, 338 service steps and 254 certified skipped steps in those examples, with the same source-equivalent application count. Thus Scan removes maintenance, while contraction additionally removes repeated selection and occurrence handling. Fewer counted operations alone do not establish which total-cost difference exceeds the practical threshold.

The competing-call families record no contracted steps. Scan still improves their total costs substantially by removing maintenance. This adverse admission control is important: the configuration choice helps even where the proposed compiler operation cannot act.

These counters substantiate the code analysis of dependency refresh over growing open terms. They do not fit an asymptotic law or prove that Indexed access is generally undesirable. The source sets have few competing occurrences, and successful update families carry unknown seeds. Ground accumulators, larger populations and different selective joins remain separate controls.

## Lifecycle and remaining uncertainty

For unary updates, feature-off specialized primary requested allocation drops from 1,036,444 bytes with Indexed access to 398,060 with Scan. Contracted Scan requests 329,725 bytes. Peak requested growth is respectively 46,996, 45,348 and 45,574 bytes. Removing traffic is not the same as reducing peak retention.

The corresponding source/input-inclusive median times are 1.075, 0.350 and 0.307 ms. Preparation, setup, complete execution/observation, engine/answer disposal and prepared disposal are included in the primary endpoint; source and input construction are separately reported. Meter/work elapsed times are excluded from timing conclusions. Requested bytes are not RSS; native compilation is excluded.

The unary example has a registered contracted-Scan/feature-off-specialized-Scan ratio of 0.772 with interval [0.663, 0.884], a practical gain. The nested example has ratio 0.895 with interval [0.874, 0.915], unresolved under the registered threshold. Ratios use paired samples and need not equal the ratio of marginal medians.

Unresolved comparisons do not establish equivalence or rejection. More precision could decide individual component choices, especially near the threshold, but the present evidence does not support a universal policy to tune. The bounded conclusion already changes materially: a large Indexed advantage cannot justify contraction without the Scan control.

## Validation and next direction

Runtime source is unchanged from the recursive semantic gate; both access paths pass the runner source checks. The [audit](s06-recursive-access-confirmation/audit.json) verifies receipts, hashes, commands and lifecycle restoration. A Clippy cleanup changes only the work companion configuration guard from constant assertions to a conditional panic, preserving runtime rejection of unsuitable builds. All 800 work payloads replay identically under rebuilt companions; the ordinary timing binaries are unchanged. The original freeze and guard-recheck freeze are retained.

This experiment has no exact/direct answer-producing control and does not decide general recursive compilation, fresh recursive locals, step effects or sustained retention. The source-specific gains and adverse cases remain inputs to later whole-architecture comparisons.

The [next investigation](S06-access-next-investigation.md) is S08 graph-observation attribution. Growing-output graph losses still have an identified temporary-copying hypothesis. Testing that explanation can change a different architectural comparison more than further tuning this accepted recurrence. T073 remains unfinished; T074 becomes active. The research goal remains open.
