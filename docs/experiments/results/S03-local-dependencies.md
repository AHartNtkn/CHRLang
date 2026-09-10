# Local admission needs binding dependencies; observation needs validation

Occurrence conflicts alone do not preserve source-priority execution: a late binding can cause both a different winner and a deadlock. Adding potential binding dependencies resolves those witnesses in the bounded model while allowing disjoint groups to proceed independently. Separately, an atomic descriptor decision does not make an unvalidated multi-field scan a consistent observation.

These are concrete protocol obligations, not a native architecture selection. Source-derived dependency discovery, multiple descriptors, memory ordering, actual source integration and total costs remain open.

## Binding information changes who may proceed

The [registered comparison](../registrations/S03-local-dependencies.md) groups applications by potential dependencies and admits the earliest enabled application within each group. One scheme includes only consumed/kept occurrence conflicts. The other also connects binding writers to potential readers, even when those readers are currently disabled. Both use the existing local acquisition protocol and are checked against an independent atomic source-priority model.

All 20 binding-aware configurations match the independent outcome sets exactly and have no reachable deadlock. Disjoint consumers remain in separate groups, with four reachable states admitting both in the no-cancellation case. This distinguishes the candidate from a control that always serializes globally, within the tested effect-bag observation contract.

Occurrence-only groups fail on late equality. The binder and a later consumer can acquire their disjoint resources. When the binder commits, an earlier consumer becomes enabled and gains priority within the token group. It waits for the later consumer's claim; that holder can no longer commit. Two cancellation configurations contain this deadlock, and three admit outcomes outside source priority. Cancelling the claim holder can resolve that particular blocked state, but requiring such cancellation would not establish normal progress.

The first runner stopped on this deadlock. Its frozen sources and failure are preserved. The second runner records shortest deadlock traces and continues all registered cells, while still requiring the binding-aware candidate to pass. No acquisition or admission semantics changed between attempts.

## The successful grouping also has a precision cost

The candidate uses complete, static potential dependencies supplied by the finite application descriptions. In the initially enabled equality case, it groups the binder with both consumers even though the occurrence-only scheme's observations already agree with source priority. Thus the additional dependency information can prevent incorrect interleavings and also serialize work unnecessarily.

This is a reason to investigate sound source-derived dependency precision and maintenance. It is not evidence that all applications must share one owner, or that the extra analysis will repay its cost. The source compiler must account for kept reads, equality-enabled candidates and future rule discovery; current group construction does not infer those facts from arbitrary CHR source.

## One atomic commit decision does not give an atomic scan

The publication model installs references to one descriptor in four separate fields, changes its status once to commit or abort, and then materializes or cleans each field separately. The four fields represent two consumed identities, a binding and a published effect. Every installation, field read and cleanup is an individual transition.

Direct scanning admits a snapshot containing three old fields and one new field: the observer reads the first three fields, the writer commits, and the observer reads the fourth. All individual reads respect the descriptor, but the combined result is neither the old nor the new transaction state. Other placements of the commit produce two additional mixed snapshots.

A status-validated scan records the descriptor's status before reading and publishes only if the status is unchanged afterward. Across both cancellation configurations, it publishes only all-old or all-new snapshots. The explored graphs contain 80 and 96 discarded-scan transitions respectively. These are possible retry obligations, not measured retry rates. The experiment does not establish starvation freedom for repeated scans or a multiple-descriptor snapshot algorithm.

Descriptor status is one atomic state-machine field in this model. A native implementation still needs synchronization and lifetime rules that make its referenced records safe to inspect. Cancellation cannot expose staged effects; cleanup cannot invalidate a reader's descriptor reference. Treating these transitions as unsynchronized ordinary writes would not implement the tested protocol.

## Evidence scope and next experiment

The completed comparison covers 40 admission configurations and four publication configurations: 3,010 reachable states in total. Admission contributes 370 states and 493 transitions; the publication comparisons contribute 2,640 states. Shortest counterexamples, both attempts and frozen inputs are retained in the [raw evidence](s03-local-dependencies/). The independent replay audit recomputes the outcome comparisons and publication witnesses.

The next package must connect these obligations to actual source execution. Use source-derived occurrence/equality dependencies and physically represented descriptors, with observations between updates, cancellation, and at least two interacting descriptors. Compare complete source outcomes against the qualified serial source path and independent expected results. Preserve ordered observations and fresh/branch identities rather than treating the model's effect bag as the whole language.

Admission and publication were registered together, but answer independent questions and count separately for breadth. Together with the initial local-claims gate, they make three packages after the continuing-service breadth review. T080 remains active because the next experiment can determine whether local admission and publication replace the existing responsibilities in a real execution path. Adaptive costs remain the strongest ready alternative; conditional equality, richer theories, broader reuse and sustained lifetime remain required. Reconsider selection at the source-integration gate or a consequential obstruction; review breadth by package four.

No timing, allocation, native correctness or general architecture superiority claim follows. The research goal remains active.
