# Wider recognition intervals help memoization, but do not beat Direct

**Strides four and sixteen qualify as faster than stride one in 239 and 240 of 256 scenarios. Neither qualifies as faster than Direct or nonmemoized separation.** The reduced recognition work is useful within memoization, but these complete costs do not justify its retained transition machinery against the simpler controls.

The [registered campaign](../registrations/S05-recognition-stride-timing.md) completes 20,480 ordinary sessions after 2,048 exact ownership-entry checks. All 7,168 contrasts pass clock qualification; the required endpoint floor is 2.1 microseconds. Full answers, phase sequences and retained counts validate. No samples were excluded.

## Complete session results

Each row has 256 comparisons. Ratios and faster/slower classifications refer to the **first named mode**, preserving the registered contrast direction.

| Timed ratio | Qualified faster | Qualified slower | Unresolved | Median ratio range |
|---|---:|---:|---:|---:|
| Stride 4 / stride 1 | 239 | 0 | 17 | 0.283–0.680 |
| Stride 16 / stride 1 | 240 | 0 | 16 | 0.133–0.755 |
| Direct / stride 4 | 212 | 0 | 44 | 0.209–0.798 |
| Direct / stride 16 | 171 | 0 | 85 | 0.444–0.899 |
| Nonmemoized separation / stride 4 | 203 | 0 | 53 | 0.279–0.931 |
| Nonmemoized separation / stride 16 | 133 | 0 | 123 | 0.588–0.999 |
| Sealed compiled / stride 4 | 180 | 1 | 75 | 0.186–1.176 |
| Sealed compiled / stride 16 | 103 | 0 | 153 | 0.336–1.167 |
| Indexed / stride 4 | 105 | 33 | 118 | 0.341–1.488 |
| Indexed / stride 16 | 18 | 77 | 161 | 0.495–1.488 |

A qualified gain requires median ratio at most0.90 with every paired block below one; a loss requires median at least1.10 with every block above one. Unresolved does not mean equal. Ratio inversion can change an even-sample median, so the table keeps the original orientation rather than treating reciprocal medians as new results.

The endpoint includes source construction, preparation, one/four alpha-renamed query uses, service/observation, output retention, cancellation and all disposal. Independent validation and its warm pass are outside the measured session. Primary builds have no allocation, engine, kernel or work counters. Compilation and process startup are not included. No workload weights or universal winner are inferred.

## The favorable sealed comparison still has a stronger control

The single qualified case where sealed execution is slower than stride four uses readable caller observations with propagation history, depth four, one query, retained output, cancellation and equal caller tags. Sealed/stride-four median ratio is1.162, with all blocks above one. Computing the reverse ratios directly gives stride-four/sealed median0.860, range0.711–0.978.

Direct qualifies as faster on that same source: Direct/stride-four median0.746, range0.648–0.873. Median session durations are38.22 microseconds for stride four,41.44 for sealed and28.32 for Direct. These individual medians are descriptive; paired ratios determine the verdicts.

Thus the isolated sealed advantage does not warrant adopting the memoized path for that source. Generic indexing has additional adverse cases, which remain visible in the full matrix rather than serving as the only control.

## Disposal is not the whole remaining cost

The [post-campaign diagnosis](s05-recognition-stride-timing/diagnosis.json) sets every candidate disposal phase to zero and leaves Direct unchanged. Resulting median candidate/Direct ratios remain above one in all256 scenarios:1.053–4.070 for stride four and1.005–1.998 for stride sixteen. This is an analytical bound on a disposal-only remedy, not a fresh timing classification or a bound on changes to execution and recognition.

The work and ownership gates explain the competing costs: wider spacing builds fewer keys but misses earlier convergence and retains more states. The implementation still owns replayable transitions. Further reuse designs may avoid different work; this campaign does not reject those mechanisms.

## Next investigation

The [portfolio review](S05-recognition-stride-timing-review.md) selects T076 for connected finite projection and source multiplicity correspondence. Existing compact-space benefits justify testing how local logical relations compose and eliminate hidden coordinates. This is a distinct mechanism, not another interval-tuning campaign.

T075 remains unfinished for call-boundary recognition, variable relevance, failed futures, eviction and sustained ownership. Keep the validated stride mechanism and its adverse controls as evidence. The full research goal remains active.

Reproduce with `python3 research/chr-reuse/experiments/recognition_timing.py --audit` and `python3 research/chr-reuse/experiments/recognition_timing_diagnosis.py`. [All contrasts](s05-recognition-stride-timing/analysis.json), raw sessions, frozen sources and binary hashes are retained.
