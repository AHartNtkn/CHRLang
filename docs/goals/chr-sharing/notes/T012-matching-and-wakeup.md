# Constructive matching, wake-up and quiescence

A complete matcher can be built from finite tuple enumeration and condition-sensitive structural tests. An incremental implementation can preserve completeness by recording every binding it reads and scheduling newly possible tuples. This supplies an implementable correctness design; its profitability and memory costs require experiments.

This is original analysis over [the conditional kernel](T011-conditional-kernel.md). It assumes finite current stores, finite terms in each live projection, immutable constructor nodes, and terminating supported guard tests. It does not introduce bindings through matching or change the guard language.

## Matching one tuple

For a rule with h head positions, enumerate tuples of occurrences with the required predicate symbols and arities. Disallow repeated occurrence IDs within a tuple. Intersect their live supports to obtain the candidate condition C. This coarse predicate index is complete even when every argument is unknown; optional constructor/key indexes need their own invalidation discipline.

Maintain a worklist of `(pattern, term-reference, support)` obligations and a conditional map μ from rule-local head variables to term references. Conditional dereference returns a partition into unbound representatives or constructor roots, retaining the exact support of each case. It does not instantiate an unbound store variable to satisfy a pattern.

For a first occurrence of a pattern variable, record its reference in μ without inspecting its constructor. For a repeated occurrence, test rigid equality against its recorded reference under the current store substitution. Identical representatives succeed. Equal constructor symbols/arities generate child comparisons. Different constructor symbols/arities cannot match. Distinct unbound representatives, or an unbound representative against a constructor, do not establish equality and may become equal after a later equation.

A constructor pattern against an unknown store representative is similarly not currently enabled. Its dependency is recorded so later instantiation can wake the match. This is suspension of matching, not implicit constructor generation. In relational addition, explicit body equations supply the constructor alternatives.

The result is `(D, μ, dependencies)`, where D includes exactly those candidate alternatives whose entire head and guard test succeeds. μ projected on each of them is the scalar matching substitution. A shared μ may map a pattern variable to different child references under different conditions. Knowing only D is insufficient to construct the RHS.

Guard tests use μ and the projected built-in information. A guard API must expose its read dependencies or be conservatively invalidated by every update to its declared built-in domain. Undeclared external reads cannot be treated as a cacheable pure test. The precise additional built-in catalogue remains separate from this algorithm.

### Structural-test termination and fidelity

Evaluate structural equality/dereference as a graph worklist, memoizing the support already processed for a node or node pair at a fixed binding version. Revisit only the remaining support. Boolean operations must be semantically exact and terminate; syntactically growing formulas without an equivalence or progress discipline do not suffice.

For a fixed finite graph and frontier, there are finitely many node pairs and support memberships. Each live projection has acyclic finite terms. Projecting a worklist item therefore gives an ordinary nonbinding structural test. First pattern occurrences construct μ, repeated occurrences enforce already-established equality, and constructor cases exhaust the possibilities. This proves completeness of the tuple test under the stated worklist discipline, without proving that the symbolic representation is economical.

## A complete conservative work-discovery algorithm

At a fixed branch version, enumerate all rule/head tuples and run the matcher, subtracting their history conditions. Together with pending equation/choice/insert/fail goals, this gives all enabled reference work. Finite arity and finite occurrence counts make the enumeration finite. An index scan that yields partway through is not a quiescence certificate until it completes.

This construction is intentionally a correctness control. For predicate occurrence counts n₁ through n_h, coarse tuple enumeration can examine up to their product before identity and selectivity checks. A program with no useful joins may still pay that cost. No claim of efficient CHR compilation follows from completeness.

## Incremental work discovery

Use the conservative enumeration result as the target for the following event-driven optimization:

1. **Insertion:** enumerate tuples containing the new occurrence in every compatible head position. Deduplicate duplicate tuple work by rule, ordered IDs and support; do not identify distinct occurrences by their arguments.
2. **Binding change:** wake suspended/cached tests that read a changed variable, binding edge or built-in domain, intersecting the subscription support with the update support.
3. **Consumption:** invalidate work depending on the consumed occurrence only where it was consumed. Positive heads gain no new partner merely because another occurrence disappears. A richer guard/domain interface must separately declare any relevant removal event.
4. **History update:** subtract the firing support from further work for that token. A token acquired in A cannot suppress B.
5. **Choice:** inherit cached facts and subscriptions through the frontier map, then register each arm's new goals. Unexecuted labels must not multiply work or answers.
6. **Failure:** cancel the affected support; retain any shared task's still-live support.

Every dereference records all traversed binding links, not merely its current terminal representative. If a previously unbound root is redirected, old subscribers must hear that change. On reevaluation they also subscribe to the new path. Registration must be race-safe: record read versions, install subscriptions, then validate those versions and requeue the stale support, or perform read-and-subscribe atomically. Apply this on alias-path resubscription too. Commit validation alone cannot rescue a suspended test that missed its only wake-up. This handles alias-sensitive wake-up without assuming a stable representative ID.

Constructor structure is immutable here. Destructive graph updates require additional version/dependency events or an ownership proof that no observer can see the change. A physical update justified only for one alternative cannot silently modify other projections.

### Wake-up completeness argument

Suppose a tuple changes from not enabled to enabled in some alternative. Its head occurrences either include a newly introduced occurrence, or were all present before. The first case is covered by insertion enumeration. In the second case, at least one failed or suspended matching/guard prerequisite must change. Each inspected prerequisite either supplied a recorded dependency or declared a conservative domain dependency; that event wakes the test. Previously skipped suffixes of a failed match are examined when the earlier prerequisite changes.

With positive CHR heads, unused-token conditions only move from permitted to forbidden for an existing token. Consumption removes heads rather than creating a matching tuple. Explicit goal processing covers additional pending work. Thus every relevant change schedules the affected tests, and every tuple that remains enabled is eventually discovered under fair service. A competing committed step may invalidate a transient opportunity before it is serviced; the language need not preserve that opportunity as another search alternative.

This is a sufficiency argument, not a claim that a particular subscription implementation is correct. Losing subscriptions during alias redirection or cancelling a task's entire support after a partial failure violates it.

## Commit validation and scheduler interaction

A queued match is not authorization to fire forever. At commit, validate occurrence membership, relevant binding/domain versions and token absence over the support to be committed. Recompute only the stale portion. A successful commit uses the same μ whose prerequisites were validated.

An implementation that repeatedly invalidates one branch's work because unrelated branches progress can starve it. Branch-conditioned versions, immutable snapshots and branch-local reservations are candidate ways to avoid that. These are backend choices; a single global version counter is a safe but potentially non-live invalidation control.

## Quiescence certificate

For a represented support E, establish both that no pending goal exists there and that a complete scan or equivalent justified index state finds no enabled unused tuple. Suspended constructor tests with unknown values are not enabled applications; they may remain as residual constraints in an answer.

Attach the certificate to the branch versions whose state was checked. Before publication, validate that those versions are unchanged. Updates outside E must not invalidate the certificate. A ground output, an empty local queue, or a shared task still scanning other alternatives is not a substitute for this check.

In a branch that becomes stable and quiescent, a finite complete scan can finish under fair servicing. In a branch that keeps changing, repeated invalidation does not establish quiescence. This separates trustworthy answers from progress of other, possibly divergent, alternatives.

## Experiments now needed

The construction supplies a correctness control and a precise optimized variant to implement. Remaining questions are the number/size of supports, duplicated structural tests, subscription traffic, alias-redirection costs, and how much post-choice computation is saved. These depend on actual representation and workloads.

Compare whole-tuple rescanning with complete dependency events, and coarse predicate indexing with any stronger index. Count matching obligations, dependency edges, invalidations, stale commits and conditional-operation costs, as well as rule firings. A benchmark that measures only smaller stores or fewer answers cannot establish the claimed shared work.
