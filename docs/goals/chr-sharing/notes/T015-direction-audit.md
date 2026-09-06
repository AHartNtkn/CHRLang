# Research questions and actual dependencies

Research remains active. Several candidate algorithms can be developed further without executing the proposed conditional-store experiment. The rows below identify that work; an open question is not assigned an implementation dependency merely because testing will eventually be necessary.

## Conditional multiset execution

**Remaining question:** Do the local projection arguments compose into preservation and finite-answer progress for the whole machine, including fresh choices, conditional equality and quiescence?

**Disposition: continue analysis.** T011 and T012 establish constituent arguments, with explicit premises. A whole-machine transition classification, invariant closure and counterexample review are still useful before implementation. Empirical condition/index overhead is separately implementation-dependent; T013 specifies one way to measure it.

## Memoized relation and derivation graphs

**Remaining question:** Can occurrence expansion and its common descendants remain shared after choices while aliases, constructor demands and active constraints evolve independently?

**Disposition: continue analysis.** MPT has task-specific variable bindings, and Angelic CHR supplies a relevant derivation-graph representation with different semantics. Develop a concrete restricted relation construction and identify exactly what broader CHR interactions add. Source results alone do not settle the adaptation. See T015-source-expansion.md.

## Named superpositions

**Remaining question:** Can correlated value alternatives support relational bindings and constraint effects without premature distribution or a whole-store collapse?

**Disposition: continue analysis and source inspection.** The pinned HVM4 rules establish label mechanics, not a CHR translation. Work through aliases, two independent choices, repeated uses, scope and occurs failure. Compare a pure substitution-passing encoding with native logical-variable services. The source requirement is semantic opacity, not any particular runtime node vocabulary.

## Ordinary local nets and richer graph calculi

**Remaining question:** Which finite service encoding realizes variable sharing, equality, suspension, choice and multiheaded resource gathering, and what restrictions simplify it?

**Disposition: continue analysis and targeted sources.** Fragment R's single-headed nonoverlap only settles occurrence selection. RMC embeds nets in the reverse direction. Richer additive nets and static graph-invariant sources remain relevant leads. A generic interpreter encoding may settle expressibility while leaving locality and useful cost unanswered; distinguish these claims.

## Compact search scheduling

**Remaining question:** Is one ticket per alternative necessary for fairness, or can finite symbolic support jobs supply equivalent progress with partial publication?

**Disposition: continue analysis.** T012's FIFO argument has a per-ticket premise. Investigate finite sealed job supports, resumable operations and fair discovery of support partitions. Required proof: an admitted finite derivation cannot be hidden indefinitely by new branches or by another member of a shared job. Cheap implementation is a later, separate question.

## Logical solver regions, tabling and learning

**Remaining question:** Which exact properties do no_c, neq, var and norm certify, and which can be reused without changing the observed program?

**Disposition: continue analysis.** The notebook definitions are available. Check rule-by-rule logical validity, residuals, interaction with host multiplicity and future equality, and exact versus consequence-only reuse. Do not presume a predicate named norm completely recognizes normal forms or that a name constraint is general term disequality.

## Compiler properties and language changes

**Remaining question:** Which static restrictions buy specific mechanisms, and can those properties instead be inferred or attached to regions?

**Disposition: continue analysis and sources.** Nonoverlap, ownership, separation, single assignment, constructor demand, deterministic subcomputations, idempotent solver regions, finite domains and resource bounds must be evaluated separately. Restricting one property does not establish the others. Compare effects on relational addition and partial-program synthesis, including both local eligibility and a language-wide design.

## Answer representation and execution-state reuse

**Remaining question:** Which residual projection is sound, and what stronger equivalence permits merging continuations?

**Disposition: continue analysis before owner decision.** T008 offers full residual multiset observation and structural alpha equivalence. Derive counterexamples and sufficient certificates for hiding existential components and for continuing from a cached state. Then ask the owner about display/API defaults; those preferences do not block mechanism research.

## Storage, recomputation and cost comparison

**Remaining question:** What shared information must each credible control retain, and which workloads distinguish storage savings, event reuse, tabling and pruning?

**Disposition: analytical comparison remains; performance ranking requires implementations.** T013 covers conditional execution against two controls but is not a comparison of all candidates. Extend the cost accounting and workload implications when the graph/net/solver constructions are concrete. No invented speed threshold or preferred implementation language is required to do this.

## Query syntax, primitive catalogue and future applications

**Remaining question:** Query-level equations/OR and exact additional built-in APIs require owner semantics; surface spelling requires owner taste or later ergonomic design. Future streams/reals remain outside the initial implementation target.

**Disposition: owner-dependent only for those interfaces.** The current research can preserve body-only equations and equality-only built-ins while examining candidate extensions explicitly. Finite-tree terms are the baseline; future coinductive applications are not evidence for changing it. Do not demand an interface choice to continue independent execution research.

## Completion assessment

Every direction above must eventually have evidence for closure, a specific empirical dependency, or a specific informed owner decision. This is an interim audit and does not meet that terminal condition. New relevant directions found during the listed investigations join the audit. Milestone status and the number of source documents do not substitute for answering its questions.
