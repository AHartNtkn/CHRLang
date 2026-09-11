# Deep matching exposes a time–memory tradeoff for retained intermediates

Proper intermediates have lower sampled lifecycle times on some deep matches, including against generated Active indexed execution. They use more heap than local scanning in every tested configuration. Shallow matches and early consumption expose substantial overhead, so this pilot supports investigating a conditional tradeoff—not selecting maintained matching everywhere.

**Next, confirm the consequential deep-match timing contrasts against both generated access and local scanning, with adverse controls.** Five samples leave the sparse case uncertain and do not establish an architectural recommendation. The research remains active.

## What this pilot measures

The [registration](../registrations/S01-prefix-lifecycle.md) fixes 480 configurations over sparse, keyed, early-consumption and mismatch sources; depths zero/32; widths four/eight; one/four changing queries; and releasing/retaining consumers. The eight eligible modes are local scan, partial and proper intermediates, generic Global Scan/Indexed, generated Global Indexed, generic Active Indexed and generated Active Indexed. Active early-consumption costs are excluded because [that policy changes the answer](S01-generated-prefix.md).

The runner reuses the existing shared source and lifecycle machinery. Rules and queries now have separate constructors, so query-input measurement does not construct discarded rules. Actual native code comes from the existing general access generator. Prepared rules survive changing queries; matched intermediates are built anew for each query and reused within that query. This does not measure cross-query reuse of intermediate matches.

The [independent audit](s01-prefix-lifecycle/audit.json) validates 960 allocation and 2,400 timing processes, complete endpoints, exact allocation pairs and final heap restoration. The 480 configurations produce 8,400 measured complete answers in total, plus unmeasured warm completions. Source, preparation, input, setup, execution, observation and disposal count in totals. Cancellation is checked and reported separately. First and full observation coincide for these deterministic single-answer programs.

Timing uses an ordinary allocator with counters disabled. Allocation uses separate counters-off builds with no phase clocks. Compilation, linked code storage, process startup, preflight, validation and harness bookkeeping are outside totals; this is not complete compilation lifecycle accounting. Allocation requests and tracked peak heap are not RSS.

## The strongest useful witness also retains a simpler competitor

For keyed depth-32 inputs, width eight, four changing queries and retained-all answers:

| Executor | Requested bytes | Peak heap above baseline | Median lifecycle time |
|---|---:|---:|---:|
| Proper intermediates | 2,634,196 | 354,204 | 1.300 ms |
| Local scan | 2,417,972 | 314,908 | 1.433 ms |
| Generated Active indexed | 1,535,729 | 224,512 | 1.993 ms |
| Generated Global indexed | 1,736,305 | 223,416 | 2.041 ms |
| Partial joins | 3,660,628 | 617,428 | 5.150 ms |

**The sampled timing benefit survives a competent generated control, but most of it is available through local scanning too.** Proper-intermediate/Active-native ratios have median 0.702 and observed range 0.581–0.760 across five paired blocks. Against local scan the median ratio is 0.870, range 0.839–0.907. Those are exploratory observations, not confidence bounds. The architectural question is whether that remaining gain earns maintained state, rather than whether intermediates can beat a slower implementation.

**The distinction becomes narrower at other sizes and reuse counts.** At keyed depth 32, width four and one retained query, intermediates are slower than local scan in every sampled pair (ratio 1.096–1.317), despite being faster than generated Active indexed in every pair (0.606–0.734). At width eight and one query the local-scan comparison straddles equality. A generated-only comparison would overstate the case for retention.

## Sparse matching needs confirmation; adverse sources already matter

**The sparse deep witness has unresolved timing variation against the strongest controls.** At width eight/four retained queries, proper intermediates request 4,037,332 bytes with 354,204 peak heap and a 1.572-ms median. Local scan requests 3,794,228/314,908 and takes 2.075 ms; generated Active indexed requests 1,530,033/224,512 and takes 1.815 ms. Yet the paired ratio ranges cross equality: 0.715–1.287 against local scan and 0.651–1.480 against Active native. The pilot cannot establish those speed differences.

**Shallow work reverses the apparent benefit.** In the corresponding depth-zero keyed case, the median paired ratio is 1.59 against local scan and 2.15 against generated Active indexed. In depth-zero sparse matching, it is 3.35 against Active native. The retained machinery cannot be justified merely because the source has multiple heads.

**Early consumption and mismatches favor simpler local discovery.** With depth-32 early consumption, width eight/four retained queries, intermediates take a 0.945-ms median versus local scan's 0.594 ms; every sampled paired ratio favors local scan. Deep mismatch likewise favors local scan in every paired sample. Intermediates can still beat generated execution on the mismatch case, illustrating why a simpler local competitor must remain in the experiment.

All timing here is sizing evidence. Five empty-clock calibrations each have a 14-ns median. Under the registered conservative total/phase-count test, 24 of 480 cells are instrumentation-sensitive. The displayed deep and shallow width-eight/four-query examples pass that test, but passing it does not remove scheduling noise or supply statistical confirmation.

## Extra memory belongs to matching, not retained output

**Proper intermediates request more bytes and reach higher peaks than local scan in all 64 matched scenarios.** Against partial joins, they request less in 40 scenarios, more in eight, and the same in 16; peaks are lower in 48 and equal in 16. They request more than generated Global indexed in 60 of 64 scenarios, with higher peaks in all 64. Against generated Active indexed, the result varies: more traffic in 38 of 48 and higher peaks in 40 of 48.

**The output endpoint is equally sized across all matched executors.** After source, input, engine and preparation disposal, retained output bytes agree with the corresponding local-scan control in all 480 cells. The deep width-eight/four-query sparse and keyed examples each retain 109,600 bytes. Different answers or output storage do not explain the matching-memory gap in this pilot.

**The measured phases locate a maintenance cost beside saved execution time.** In the deep keyed witness, local scan and proper intermediates have identical 3,774-byte prepared-rule allocation. Query setup requests 1,294,048 versus 1,479,520 bytes across the four queries. Execution requests 883,968 versus 914,720 bytes. Proper intermediates therefore allocate more even while avoiding repeated structural traversal. Median setup time rises from about 0.233 to 0.485 ms, while execution falls from about 0.793 to 0.315 ms. Phase medians need not sum to the median lifecycle.

This is evidence about the current retained-prefix implementation. It does not prove the additional allocations unavoidable. Its extra responsibilities include stored prefix bindings, readiness/condition maps, watcher and incident ownership, invalidation and terminal-extension search. Local scanning avoids maintaining those structures. A future repair is worth testing if attribution shows it can materially change the time–memory comparison; reducing counts alone is not the objective.

## Next experiment and portfolio decision

**Confirm the keyed and sparse deep contrasts with strong simple controls before expanding implementation.** Freeze the current binaries and compare local scan, proper intermediates and generated Active indexed on both widths and reuse levels. Keep Global generated access for the early-consumption adverse control. Include shallow matches, mismatch and early consumption so confirmation cannot become a favorable-only result. Preserve releasing and retained consumers where ownership can affect the conclusion.

Use the pilot's paired variation to register an adequate fixed confirmation sample count, retain all samples and report intervals plus the practical 10% threshold. Inspect total and phase variability before deciding whether longer measured sessions or another clock design are necessary. Keep allocation evidence separate; no new metered matrix is needed merely to repeat identical ownership observations.

**Compact runtime confirmation remains the strongest ready distinct alternative.** Both candidates now have consequential runtime uncertainty. The retained-matching pilot newly exposes a benefit against native execution that may mostly come from simpler local scanning; confirming that distinction can directly determine whether maintained machinery deserves complete-path integration. That earns one bounded follow-through ahead of compact confirmation. Demand and integration costs remain required; they receive no negative disposition from this ordering.

This is package three after the last full portfolio review. The next confirmation result or obstruction must trigger a full review, counting measurement qualification or attribution work as a package if that is what occurs first. Complete architectures, broader joins/subscriptions, language alternatives, sustained lifetimes and held-out sources remain unresolved.

## Validation

Both builds pass the 90-case prefix gate and the existing 80-case lifecycle smoke gate. The matrix checks independent complete answers, setup-only/one-advance cancellation, contiguous metered ownership, releasing-query baselines and final restoration. Separate gates verify prepared reuse after cancellation and retained answers after producer disposal. The audit reconstructs every randomized block and exact allocation pair from frozen evidence.

Strict scoped Clippy passes. The structural-prefix source regression passes after separating source construction, and generated-prefix regressions run in both counter configurations. Prior source, generated/activation and intermediate-lifecycle audits remain valid against their frozen archives. No runtime engine algorithm or independent reference implementation is changed by this package.
