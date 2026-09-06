# Delayed splitting and independent product search

These are additional execution directions, not a selection of architecture. The source assessment and the proposed CHR constructions below are separate: the latter are original paper arguments requiring implementation validation before an operational claim.

## Source findings

Lopes, Santos Costa and Silva's [Extended Andorra Model / BEAM paper](https://arxiv.org/pdf/1101.6029), §§2–4 and §8.3, describes nested conjunction and alternative boxes, propagation of ancestor constraints, promotion from a sole remaining alternative, and splitting that duplicates surrounding goals. Reduction introduces alternatives from matching clauses. The default control favors deterministic work and delays splitting. The evaluation discusses cases where delayed splitting performs speculative work indefinitely, and where producer annotations improve behavior by splitting earlier. Thus the paper supports delayed splitting as a mechanism with a control tradeoff; it does not establish this project's fair explicit-OR semantics.

Mateescu, Dechter and Marinescu's [AOMDD paper](https://arxiv.org/pdf/1401.3448), §§2–4, 7 and 9, studies finite graphical models with finite variable domains. AND nodes represent decomposition; OR nodes represent assignments. Its canonical representation is relative to a pseudo tree, and its size bounds depend on the model's width. The finite model assumptions matter: an unbounded recursive CHR computation is not one fixed finite constraint network. A finite snapshot of choice supports can use such representations, but the source's bounds do not become bounds on the whole evaluator.

## An explicit-OR Andorra adaptation

Give each explicit source disjunction an alternative box. Ordinary CHR rule competition still commits to one permitted application; it creates no alternative box. A conjunction box contains active occurrence identities, pending work, scoped bindings and propagation history. Child boxes inherit ancestor state and record conditional changes. A child's equation cannot mutate an ancestor variable for sibling alternatives.

The opportunity is to execute an ancestor operation once while its inputs remain opaque across descendants. For example, a pending `carry(X,s^W(z))` can undergo a linear catchall expansion sequence independently of an alternative `X=a | X=b`. The same W expansions can serve both descendants when they carry X by logical handle. Constructor inspection eventually needs contextual resolution. This is the owner's post-choice sharing example, realized as delayed distribution rather than immediate whole-store copying.

External-variable binding alone is an insufficient permission test. Consider common `p(a)` and alternatives containing `q(a)` versus `r(a)`, with `p(X),q(X) <=> hit(X)`. The left arm consumes the common p occurrence; no logical variable needs binding. Promoting that consumption unconditionally would prevent a permitted computation in the right arm. Likewise, propagation history and newly posted constraints can be conditional without any external binding.

A sufficient lifting certificate for an operation over descendant support C is:

1. Every projected alternative in C has the required occurrence identities and the same rule/ordered-head token, with a nonbinding match valid there and an entailed guard.
2. The selected operation agrees with the committed policy in each projection. Alternatively, a proved commuting transformation justifies changing the policy locally; merely being enabled is insufficient for preserving a previously fixed policy.
3. Every observable difference in bindings, membership, token history or fresh body work is retained under its exact support. An effect may move to the ancestor without a condition only when valid over that ancestor's entire live support.
4. Shared expansion preserves source event provenance and fresh-variable scope. A reused immutable expansion template does not imply that distinct applications share fresh logical variables.

Under these conditions, lifting is the conditional machine's pointwise transition argument applied to a box representation. This identifies the common semantic obligation without pretending the representations have equal cost. A tree of boxes may avoid support algebra for nested alternatives, but repeated overlapping contexts can require distribution, duplication or a DAG extension. A conditional DAG can retain those intersections directly at the cost of support operations.

A sole surviving arm can be promoted after actual failure has excluded its siblings. Divergence is not failure. Two successful arms with identical current substitutions are still separate alternatives until a sound observation or continuation-equivalence criterion applies. Residual constraints remain part of successful CHR observations; an empty goal list alone is not a sufficient success certificate.

## Scheduling delayed splitting

“Do all deterministic work first” is not an admissible unrestricted fairness rule. If a sibling generates deterministic work forever, an explicit alternative awaiting attention must still progress. Give box operations finite quanta and age queued splitting work alongside other source operations. Administrative propagation and copying must also be resumable. The sealed-job scheduler in T016 supplies a sufficient progress discipline; a preference for cheap deterministic steps can operate within bounded quotas or an aging policy.

This adaptation closes the conceptual question of whether Andorra-style delay is compatible with explicit OR and CHR resources: it is, with resource-aware contextual effects and fair scheduling. It leaves empirical questions about box/DAG allocation, detection cost, speculative work and the profitable split policy. These should be compared on both early-failing producers and long opaque continuations. A benchmark that only rewards late splitting would omit the source paper's main counterpressure.

## Persistent independence for AND decomposition

Variable disjointness is insufficient. `p(a),q(b)` can participate in `p(X),q(Y) <=> r(X,Y)`. Even a currently disconnected store can become connected when a body posts a new partner. Independence must cover future resource interactions.

A sufficient conservative region certificate is:

- Each active/pending occurrence belongs to exactly one region. No permitted rule application has heads in different regions, including retained heads.
- A body's fresh occurrences and writable logical variables remain in its region. Regions share only immutable ground parameters; distinct regions cannot later acquire a writable alias.
- Guards and builtins have no hidden mutable dependencies across regions. Propagation tokens and pending private operations are region-local.
- Fresh occurrence and variable supplies are disjoint, or are renamed apart when composing results. The chosen committed policy within a region is independent of interleavings in other regions.

A static sufficient implementation can use disjoint closed predicate families. Another can carry a rigid region key through every head and body and require all heads of each rule to have the same key. Such a key must be a proven distinct ground value, not a free variable that could later unify. A region key alone is insufficient if another unkeyed predicate joins regions or a body exports a variable.

Shared read-only ground facts require care: a CHR occurrence can be retained by one rule and consumed by another. Replication is justified only for a certified persistent fact interface, with propagation multiplicity accounted for. Calling an occurrence read-only by convention is not a certificate.

## Product argument and enumeration

For certified regions, compose states by disjoint union of pending work, occurrence multisets and histories, and union of disjoint substitutions. Each source transition affects one component; its prerequisites are unchanged by another component's transition. Therefore transitions in different components commute up to fresh renaming. Induction gives a projected execution in every region, and any finite interleaving of regional executions gives a permitted global execution under the stated policy condition.

A global answer consists of one quiescent nonfailed regional answer from every region, composed with fresh local variables renamed apart and outputs reconstructed in their original positions. The full residual multiset is the union. A region's residual answer must remain stable against every future action outside that region; this follows from the persistent certificate, not merely current quiescence.

Solve each region once and retain its answer stream. Enumerate products fairly: schedule producers fairly while revisiting finite index boxes, for example all tuples with indices at most n at stage n. Emit a tuple only when all its entries are available; do not block all other work waiting for a missing entry. Every tuple of answers produced after finitely many steps is eventually emitted. Nested loops that exhaust the second stream before advancing the first do not meet this condition. Explicit alternative multiplicities survive the product; final observation dedup remains separate.

Failure of one regional alternative excludes combinations containing it. Exhaustion with no regional answers establishes an empty product; a producer that has not yet answered does not. Infinite production can require unbounded caches if later combinations need old answers. Recomputing or spilling those answers is a storage policy with measurable cost, not a semantic reason to omit combinations.

This provides sharing distinct from both memoized calls and opaque event expansion: an independent factor can be solved once for many combinations with another factor. Enumerating N distinct concrete output tuples still costs at least N output events. Returning a symbolic product would change the user-facing answer contract and needs a separate owner decision.

## Remaining questions

The sufficiency arguments above can proceed to implementation without a new semantic decision. The useful empirical questions are how often inferred regions exist in addition/SK/lambda workloads, how certificates cost relative to saved work, whether conditional contexts permit larger independent factors, and how product caching affects memory and latency. A mandatory region language or a factored-answer interface needs an owner decision only if proposed for adoption. Neither decision blocks other research.

A dynamic decomposition algorithm can conservatively recompute an interaction overapproximation, but must include reachable future predicate interactions and alias exports. Supporting temporary independence followed by reunion needs an additional reconciliation protocol; the persistent result here does not justify it. This is an independently actionable analytical question and remains open.
