# S04: paired cost of rejecting incompatible roots before copying

This comparison determines whether the shared matcher correction materially changes the restoration pilot's interpretation. It does not select a restoration architecture. The alternative next investigation is S05 stable-identity reuse; this bounded attribution test comes first because the measured 36,864 rejected copies could dominate the mutation result.

## Hypotheses and controls

The correction should reduce mutation lifecycle time and allocation by avoiding copies of already incompatible entry bindings. It may add cost on compatible candidates, especially when following aliases repeats traversal. Retained-store and work-between-choices sources check effects outside mutation. An unchanged Indexed implementation controls for unrelated build or timing differences.

Compare the exact archived `s04-matcher-copy/before-lib.rs` with the current corrected restoration library. Build each in an isolated standalone Cargo package with the same package identity, edition, dependencies, release defaults and lifecycle harness. The only library difference is the documented root precheck and diagnostic instrumentation; diagnostics are disabled in primary timing and allocation builds. Preserve the original S04 binaries. Record source, generated package, dependency and binary hashes, commands and toolchain before comparative runs. Native build duration is operational metadata, not an isolated compiler-cost measurement.

## Exact source matrix

Use `examples/lifecycle.rs` with these six families:

| Family | Source configuration |
|---|---|
| mutation | Original N128, depth3, edits32, work2 |
| retained | Original N128, depth3, edits0, work2 |
| work | Original N8, depth3, edits0, work32 |
| compatible-small | Kept tag with atomic key, 32 consuming item partners with the same key |
| compatible-large | Same source with key `f` nested 64 times around an atom |
| compatible-alias | Seed rule establishes 64 successive variable aliases; tag and 32 partners refer to the first variable |

Compatible sources post one `seen` residual per item, alternating two joint unknowns that are also outputs. They have one answer. The alias seed head binds every query variable before its body posts equations; these are query aliases, not fresh body locals. Independent source evaluation and an explicit complete expected answer check the aliases and residual multiplicity. Gate both seeds 0 and 7. No source choice or key mismatch is introduced into compatible controls.

Four modes: Copy, Trail, Checkpoint1 and existing Indexed. Two ruleset reuse counts: 1 and 8 changing queries. Two matcher versions, six families, four modes and two reuse counts yield **96 cells**. Run five ordinary-allocator timing repetitions and two separate allocation repetitions: **672 processes**. This deliberately excludes root replay and longer checkpoints from the paired matrix: the immediate decision is whether a common matching defect materially affects the pilot. Their corrected policy costs remain a separate obligation if this correction matters.

## Measurements, bounds and analysis

Use the established harness: two warmup queries, timed preparation, per-query setup, joint execution/observation through exhaustion, engine and answer disposal, then a separate run to first answer and cancellation and final prepared disposal. Validate full answers outside timing. Primary batch total charges prepared disposal once and excludes the separately reported cancellation phases. Report first-answer latency separately. Query construction, oracle, validation, process startup and native compilation are excluded. Setup boundaries differ from Indexed as documented in the original pilot; compare lifecycle totals.

Use serial processes pinned to the lowest available CPU, seed 20260909, shuffled cells per repetition with randomly ordered adjacent before/after pairs. Start with timing, then allocation diagnostics. Each process has a 60-second wall limit, 1 GiB address-space limit and the harness's 100,000 engine-advance bound. A cutoff stops the matrix for diagnosis. No silent retries or source changes. Five repetitions are a bounded pilot, not population-level statistical confidence.

For each mode/family/reuse report all five paired after/before runtime ratios, their median and range. A practical timing change requires a median at least 20% away from one and every pair on the same side of one. A separated smaller effect remains reported; overlapping or contrary repetitions are inconclusive for that claim. No pooled workload score. Treat a practical regression on any compatible family as consequential even if mutation improves. Check unchanged Indexed controls before attributing timing shifts to the matcher.

Allocation diagnostics measure requested heap bytes, not RSS. Require exact repeated allocation totals and phase readings, complete allocation restoration after prepared disposal, and compare cumulative requested traffic and incremental peak live heap separately. Work-count evidence is the existing independently gated diagnostic, not timing counters.

If mutation materially improves without a practical contrary regression, keep the correction and update the bounded S04 interpretation, then assess which unmeasured policy comparison is still consequential relative to S05. If compatible or alias overhead is practical, investigate its actual traversal and the smallest justified policy correction before claiming a net improvement. If timing is inconclusive where it could change that decision, prospectively register further evidence. This matrix does not discharge broader S04, the remaining sequence, or the research goal.
