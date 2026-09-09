# Finite structural spaces with path equality: source gate

This package tests whether finite constructor descriptions can be intersected and constrained before enumerating their values. It establishes semantics and a working mechanism; it registers no timing comparison or architectural speed claim.

## Denotation and source contract

An acyclic finite grammar has states with zero or more constructor transitions. A transition contributes a constructor applied to independently chosen child values. Reusing a child state means sharing its description, not correlating the choices at different occurrences. Union is set union for value membership; product constructs all child combinations.

A request names one source state, zero or more membership-filter states and equalities between finite child-index paths from the root. A value is accepted exactly when it belongs to all named states, every mentioned path exists, and each equal path pair selects identical ground subtrees. Missing paths fail even when both paths are the same. Equality of a term with its proper subtree has no finite solution.

Source multiplicity is distinct from membership: count derivations in the first state by summing alternative counts and multiplying independent child counts. Filters are predicates and do not multiply this count. Emit each distinct accepted value with its exact source count; expanding that count gives raw answers for the finite pure generator fragment. Checked arithmetic reports overflow, never an invented count. This is not general CHR consumption, existential projection, rational trees or a language-semantic adoption.

## Candidate and controls

The candidate represents demanded term positions, merges positions required equal, and propagates grammar requirements through constructor choices. It explores intersections lazily instead of enumerating each input language first. Duplicate membership proofs may converge to the same value: exact value deduplication is initially explicit and its retained memory remains a cost to investigate. Source multiplicity is computed independently of membership proof branching.

The control independently enumerates each small grammar with owned terms, filters by set membership and direct path traversal, and counts original derivations. A separate source-correspondence test generates actual CHR constructor alternatives and path walks and compares complete raw outputs with the independent reference interpreter. No reference implementation is modified.

## Required tests and bounds

Test overlapping and empty unions, products, duplicate transitions, shared descriptions with independent occurrences, path equality across nested terms, transitive equalities, missing paths, arity clashes and finite-tree cycles. Compare exhaustive small grammar/request combinations, including redundant filters. Test partial service with zero/one budgets, resumption and cancellation by dropping a live search. Establish a selective witness where a compact description denotes many values but the solver avoids enumerating them; include an unselective witness where output work remains.

The initial gate uses a 100,000-service-step completion bound on exhaustive small tests, a 10,000-step bound on the large selective witness and explicit expected answer bounds. An unfinished search reports progress, not exhaustion or empty language. Each step processes one pending grammar requirement, branching over that finite state's transitions, or emits/checks one complete value. This bounds service units, not wall time or arbitrary state width. Full owned term materialization and grammar validation are not incremental in this gate.

## Interpretation and next selection

Agreement establishes this finite fragment only. Mechanism diagnostics may count expansions, duplicate observations and retained frontier states; they are not timing. Compare unreduced and structurally reduced descriptions where redundant proofs matter before a broad cost conclusion. A consequential defect is repaired and rerun. Source eligibility, richer projection/theories, reusable query artifacts and complete lifecycle costs remain required.

After this gate, select the bounded cost/reduction comparison against checkpointed restoration and reunion as the strongest distinct ready alternative. Compact solving has priority now because it could avoid enumeration and lacks a direct finite-path trial; that is a priority judgment, not evidence against restoration or integrated execution.
