# Conditional-kernel research review

This pass supplies constructive local arguments for conditional finite-tree unification and CHR updates. It also isolates a useful relation fragment and a logical solver interface. The result narrows the implementation problem, but does not yet establish efficient matching, complete wake-up or fair shared scheduling.

## Evidence gained

- [Conditional kernel](T011-conditional-kernel.md): projection, conditional reachability, finite-tree binding and supported occurrence/history updates, with local preservation arguments.
- [Local compilation and solvers](T011-local-compilation-and-solvers.md): commuting Apply steps, a relation fragment supporting unknown arguments, a conditional net-backend contract, and distinct authority for solver calls, answers and learned consequences.
- [Source resolution](T011-source-resolution.md): full CW447 recovery and exact projection/consumer-filtering conditions from full TCLP theory.
- [Scalar unification source](T011-unification-source.md): inspected original equation-transform foundation and retrieval ledger.

These are original written arguments where explicitly labelled. They have not been mechanically proved or implemented. The source papers do not establish the combined CHR construction for us.

## Adversarial review

A separate agent reviewed the kernel after resolving the source questions. It identified two essential preconditions: edge supports imply both endpoints' existence, and rule application receives a conditional matching witness in addition to an applicability condition. Both are now explicit. The review also clarified constructor clash, fixed-batch unification scope, and the distinction between the frontier universe and live alternatives.

The reviewer judged the result to advance beyond a pointwise restatement: labelled reachability constructs an occurs condition, supported edge installation constructs bindings, and Boolean membership/history updates construct rule effects. Matching, its witness, wake-up and quiescence remain specified interfaces rather than implemented algorithms. There was no remaining counterexample to the local lemmas under the stated premises; this is bounded human-style mathematical review, not formal certification.

The relation/solver analysis was independently derived by another agent and integrated by the PM. The commuting lemma is deliberately limited to Apply, which posts equations. It does not assert independence after those equations execute. A local-net source fragment simplifies arbitration but does not automatically provide a correct equality/search service.

## Completion audit for the active research objective

The previous goal turn was progress: it changed the authoritative comparative dossier and source ledger. This turn is also progress: it adds constructive arguments and closes the narrow reference-definition access gap.

Research is not exhausted, and implementation is not yet the only useful next action. The remaining algorithmic questions are concrete enough for another design pass:

1. Construct conditional matching witnesses and a complete wake-up/index discipline; account for nonlinear heads and changing aliases.
2. State a schedulable work representation and progress assumptions, including contention on common work.
3. Specify how these services could be supplied to relation graphs or a local-net backend; distinguish a proved source property from backend engineering.
4. Derive operation counts and adverse families sufficient to make bounded prototype questions falsifiable. Do not choose numerical success thresholds from imagined measurements.

Those questions can still be investigated without adopting a language restriction or writing an evaluator. Completing them should expose which remaining uncertainties genuinely require experiments, and what precise scope those experiments need.

The historical source and broad citation-inventory gaps are documented. They are not evidence that another citation will supply a missing combined theorem, and they do not justify an unbounded bibliography exercise. Revisit a source when a specific candidate claim depends on it. No claim of exhaustive literature coverage is made.

The active goal remains open. No implementation, architecture adoption, or further intent decision is required at this checkpoint.
