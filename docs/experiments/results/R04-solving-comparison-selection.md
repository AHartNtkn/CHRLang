# R04: distinguish inference from solver organization

A useful solver comparison must match domain inference before attributing a difference to representation or learning. The next full R04 implementation has lower priority than the consuming class/index question in R02. R04 now has a checked finite lowering, but ranking two finite solvers does not yet settle a broader architecture choice.

## What source research changes

Support clauses can make SAT unit propagation enforce binary arc consistency: each remaining value has a compatible value in the neighboring domain. Gent also relates DPLL search and native maintaining-arc-consistency search under corresponding branching and without pure-literal deletion. Comparing native arc consistency with only conflict clauses would therefore confound inference strength with execution organization. [Gent, *Arc Consistency in SAT*, §§2–3](https://frontiersinai.com/ecai/ecai2002/pdf/p0121.pdf).

A decisive witness already fits the checked fragment. Choose `X,Y` from three atoms and forbid `(X,Y,a0,b)` for every value `b`. A combined table or support encoding can exclude `X=a0` without assigning `Y`; individual conflict clauses cannot unit-propagate that exclusion initially. Combining same-endpoint constraints for inference is valid only if answer reconstruction retains every original forbidden occurrence.

Learning is also not exclusive to an entirely Boolean representation. Domain propagators can explain deductions to clause-learning search. If learning pays, that finding can support a hybrid rather than select CNF as the language's universal representation. [Ohrimenko, Stuckey and Codish, *Propagation via Lazy Clause Generation*](https://research.monash.edu/en/publications/propagation-via-lazy-clause-generation).

Related queries can reuse a solver through temporary assumptions. A reuse comparison must distinguish persistent preparation and valid learned consequences from query-local enumeration exclusions; exclusions for one query must not suppress answers in another. The same comparison must let the native control retain compiled tables and indexes. [Eén and Sörensson, *An Extensible SAT-solver*, §2](https://ai.dmi.unibas.ch/research/reading_group/een-sorensson-sat2003.pdf). This is evidence for the mechanism, not a current library/API selection.

Blocking every returned model can accumulate clauses, but blocking is not mandatory for SAT enumeration. Work on disjoint projected enumeration demonstrates a different organization combining chronological enumeration and learning. A poor blocking result would therefore concern that implementation, not all SAT enumeration. [Spallitta, Sebastiani and Biere, *Disjoint Projected Enumeration for SAT and SMT without Blocking Clauses*, §§2.6–5](https://arxiv.org/html/2410.18707v2).

## Consequential future contrast

Compare a native binary-table solver with bitmask domains, indexed supports and reversible arc-consistency search against a support-CNF solver with learning. Conflict-only CNF can serve as an encoding control, not the sole SAT representative. Match branching in a mechanism comparison or identify heuristics as part of each complete algorithm.

Use arbitrary asymmetric tables, propagation chains and globally inconsistent but arc-consistent networks. Inequality-only coloring does not exercise all of these differences. Initial-propagation cases expose preparation and inference costs. Repeated related contradictions expose learning. Weak constraints with many answers expose enumeration and reconstruction. Changed supplied assignments test actual query reuse. These predictions remain unmeasured.

Exactly-one value constraints are needed to preserve assignment counts in a one-hot encoding. Projection must include substituted residuals as well as selected variables. Partial SAT models cannot simply become nonground source answers: the compiler must recover their represented source assignments and full observations exactly.

A competent compiled CHR control is additionally required to rank eliminating rule execution itself. Neither the Cartesian oracle nor the copying reference supplies that performance control. Charge preparation, propagation, decisions, learning/explanations, enumeration, output reconstruction, retained storage and cleanup for all competitors.

## Priority relative to the strongest alternative

[R02's consuming integration entry](R02-consuming-integration-entry.md) tests the representation used by equality, activation and consuming rules across a broader source path. The proposed R04 contrast primarily chooses a backend for the already eligible finite relation. Without an adopted workload distribution or a concrete architecture decision depending on that backend, there is insufficient reason to prioritize its implementation over R02.

Keep the R04 gate and this source-backed comparison ready. Reconsider implementation when global inference, learning, or reusable finite logical regions could change a surviving architecture recommendation. R04 does not depend on R02 succeeding; its present disposition is lower expected decision value, not an external blocker or final closure.
