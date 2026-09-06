# Coverage review: decomposition remains independently actionable

The goal is not ready to close. The new constructions substantially advance the earlier open questions, but the review found two relevant mechanisms that need direct investigation: Extended Andorra execution and AND/OR decomposition. Neither depends on implementing T013.

## Reviewed claims and a correction

The asynchronous scheduler has a coherent finite-work progress argument when accepted supports/snapshots are sealed, source-state ownership is disjoint, and recursive source evaluation is not hidden in a primitive. The composed conditional proof states initialization and all transition cases. These are paper arguments, not executable validation or independently certified formal proofs.

The finite net-service compiler needs a distinction between immutable encoded data and channel/control endpoints. Only the former can freely receive fan/eraser insertion. The service note now explicitly requires linear use of channel endpoints and active continuations unless a separate protocol justifies otherwise. This makes the advertised fixed request/reply topology an enforced compiler premise rather than an assumption hidden inside generic wire handling.

The solver interpretation and state-equivalence arguments keep logical formulas, residual multisets, and execution histories distinct. In particular, regular structural constraints cannot replace equality between repeated holes; and final-answer equality alone cannot justify continuation reuse.

## New coverage finding

Searches for `Extended Andorra Model BEAM sharing computation deterministic promotion or split constraints` and `AND OR search constraint networks decomposition context minimal graph Dechter Mateescu` found primary sources directly addressing delayed splitting, external-variable dependencies, and decomposition-based search. The earlier notes mention functional-logic sharing, conditional stores and tabling, but do not investigate these mechanisms directly.

The source leads are [A Design and Implementation of the Extended Andorra Model](https://arxiv.org/abs/1101.6029), [AND/OR Search Spaces for Graphical Models](https://www.paradise.caltech.edu/~mateescu/papers/andor_search_spaces.pdf), and [AND/OR Multi-Valued Decision Diagrams](https://arxiv.org/abs/1401.3448). Their applicability requires full inspection of assumptions, not importing their search semantics or static-graph bounds.

For CHR, a variable-sharing graph alone cannot certify decomposition. `p(a), q(b)` has no shared logical variable, but `p(X), q(Y) <=> r(X,Y)` joins and consumes the two occurrences. Future rule bodies can also introduce partners across an initially disconnected partition. A valid certificate must cover resource interactions and future effects, not just present aliases.

## Required continuation

Investigate an explicit-OR adaptation of Andorra boxes with conditional external bindings and CHR resource effects. Separately derive a sufficient persistent independence condition for AND/OR product decomposition, including residual answers, multiplicity, late joins and fair enumeration of products. Compare these with the existing relation graph and logical solver candidates. The current direction audit must retain these as actionable research until that work is done.
