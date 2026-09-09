# S04: corrected resident replay cost pilot

Compare the corrected restoration paths through complete owned answers and disposal. This exploratory pilot can change the restoration policy investigation; it cannot select a complete architecture. Registered before comparative runs, following source revision `079da005f` and the [resident replay source gate](../results/S04-resident-replay-source-gate.md).

## Questions and controls

H1: preserving the sole working state makes replay competitive on deterministic sources without retaining obsolete checkpoints. H2: checkpoint intervals expose opposing reconstruction and snapshot costs under competing branches. H3: preparation reuse and source shape can change the preferred policy. H4: cancellation and allocation retention can oppose throughput. Each is a hypothesis, not an expected winner.

Compare copy, trail, replay, checkpoint1, checkpoint4, checkpoint16, Indexed and Indexed-COW. These are existing implementations, not a new baseline. All six restoration modes share source matching; Indexed controls include their own representation and discovery costs, so their differences are complete path comparisons rather than isolated restoration attribution.

Use the unchanged `lifecycle.rs` sources: linear, small, retained, mutation, work, deep, spine, early, late, compatible-small, compatible-large and compatible-alias. The source freeze fixes every rule, query and size. Default size/depth/edits/work is 8/3/0/2; linear is 64/0/0/16; retained is 128/3/0/2; mutation is 128/3/32/2; work is 8/3/0/32; deep and spine have depth6; early/late are 64/4/8/16. Compatible sources separately exercise small, depth64 constructor and 64-link alias keys. Compare one and eight changed queries per prepared owner, seeds 0 through reuse minus one. These fixed families expose mechanisms; they are not workload weights or an exhaustive sensitivity sweep.

## Measurement and correctness

Run four separately built release binaries: ordinary, allocation-meter, COW ordinary and COW allocation-meter. No replay-diagnostic or engine/kernel metrics feature is enabled. Freeze source and binary hashes before samples. Run the independent scalar full-answer gate at seeds0/7 for each family and supported mode in every binary; run package tests, strict Clippy and formatting. Every measured query additionally compares complete raw answers with the independent oracle outside measured phases. The reference interpreter is untouched.

The primary endpoint sums preparation, each query's setup, execution/owned observation, engine disposal and answer disposal, then prepared-owner disposal. Report the phases separately. First observation includes setup and preparation for the first measured owner; code and allocator have already been warmed. Cancellation is a separate first-answer lifecycle on the same owner after the complete queries. Report it separately, without counting its work as full-query throughput. Consumer answers are retained until query end.

Source/query construction, independent oracle evaluation, validation, process startup and compilation are outside the primary sum. Validation may affect allocator/cache state between phases. The prepared owner is dropped after the cancellation trial; its final release is charged to the primary endpoint. Disclose this ordering. Compilation is not credibly isolated and no compilation-inclusive superiority is claimed.

Allocation readings measure requested heap traffic, live bytes and peak live bytes, not RSS. Require identical readings across the two diagnostic repetitions and restoration of the preparation live baseline after final disposal. Meter elapsed times never support speed claims. Use the existing separate 72-cell work gate for mechanism evidence; it is not timing.

## Exact run and limits

The driver is `research/chr-restoration/experiments/resident_cost.py`; run `build`, then `run`. The independent audit is `summarize_resident_cost.py`. Raw receipts live in `docs/experiments/results/s04-resident-cost/`.

There are 12 families × 8 modes × 2 reuse counts = 192 cells. Use five ordinary repetitions and two allocation repetitions: 1,344 fresh serial processes. Each process executes two validated warmups before fresh measured preparation. Randomize each complete repetition block with Python Random seed20260909, ordinary blocks followed by allocation blocks. Pin samples to the first available CPU; record platform, affinity, toolchain and source revision.

Per sample: 1 GiB address-space cap, 100,000 public service calls, 60 seconds wall time except mutation/root-replay at 600 seconds. The latter preserves the existing runner's bounded allowance for repeated reconstruction of mutation-heavy states; it is not inferred from this pilot's results. Gates allow mutation180 seconds; builds/Clippy120 seconds. Stop on any failure, retain the receipt and diagnose it before resuming. Never treat a timeout as a loss or restart a process merely because observing it timed out.

## Interpretation and next selection

Report medians and full five-sample ranges, requested traffic and peak growth; no formal gain/loss classification or universal ranking. Compare checkpoint policies within each source/reuse cell, then strongest complete controls. Do not compare historical wall times as paired attribution for the resident change.

A credible favorable regime requires an adverse/lifetime challenge before policy selection. A consequential loss requires separating reconstruction, snapshot copying, discovery and output costs, then investigating a plausible repair. Overlap remains unresolved if it could change a policy. A cutoff requires progress diagnosis and a prospectively justified bound or analytical limit. Register any confirmation and practical threshold after this exploratory evidence, before its samples.

At the pilot boundary compare actual temporary separation/reunion against integrated dependency repair, considering the architectural choice each can change and its remaining implementation cost. More checkpoint tuning requires a consequential unresolved question. Reunion, adaptive splitting and broader lifetime remain independent required work regardless of this pilot's result.
