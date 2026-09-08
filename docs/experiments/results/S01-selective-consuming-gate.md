# Selective discovery now has a consuming source comparison

Direct and retained lowerings now execute a structural join with consuming resources, and both agree with independent source execution. The direct control uses the available composite index. This establishes a fairer S01 comparison than asking retained pairs to avoid the required enumeration of every output pair; lifecycle costs remain unmeasured.

## The source and the competing mechanisms

The new join requires `left(K,f(X))`, `right(K,g(X))` and `request(K,R)`. Equal keys alone are insufficient: the payload constructors and their established child values must agree. Pattern matching does not bind unknowns to create a match.

One source propagates a receipt for every eligible pair. The other keeps the left row and request while consuming the right row. Source-order matching gives each right resource to the first eligible left. A lower-priority acknowledgement resumes the serial driver after eligible applications finish. Requests, replacement, insertion and ground binding can therefore exercise both stable and changing joins.

Direct execution indexes right rows by `(resolved key, established constructor child)` and enumerates eligible partners in source order. It does not test every right row in a large key bucket. Retained execution additionally stores matching pairs and reverse incidence; updates change incident pairs and requests visit the retained relation. Direct execution does not build the left composite index needed only by retained insertion.

Keys and payloads both carry variable dependencies. A binding detaches every affected live row, changes the binding and reattaches those rows. Payload construction can expose a previously unknown `f(X)` or `g(X)`, and late child equality can make previously distinct values agree. Stored receipt terms preserve their unknown handles so later binding affects final observations as it does in the source.

## Independent correctness evidence

A 144-configuration grid varies row count, key groups, payload groups, replacement count, propagation/consumption and driver placement. The one-row settings naturally coincide in some dimensions; this is a configured grid, not 144 unrelated workloads. Every configuration is checked against independent owned-syntax source execution and both generic Global controls, Scan and Indexed.

Both lowerings also run with one-step and larger-quantum resume. Directed cases cover broad versus sparse payload dependencies, whole-payload structural revelation, late receipt values, repeated requests, insertion after consumption, duplicate rows, zero requests, missing matches, blocked replacement, cancellation/reused preparation and joint unknown outputs.

In the explicit shared-resource case, four left rows and three right rows establish 12 retained pairs. Consuming the three right resources emits exactly three receipts and invalidates all 12 pairs. No right row or retained pair remains. A second request cannot reuse a consumed resource.

Two deliberate faults are detected: omitting payload dependencies and failing to consume right resources. Source-priority changes are rejected by the checked lowering, as are unsupported rebinding scripts. These tests exercise semantic outcomes, not only internal labels.

Default and counter-free gates pass; strict Clippy passes in both configurations, formatting passes, and the full compiled-package suite passes. Fresh diagnostic runs reproduce the same work counts. [Validation receipts](s01-selective-gate/validation.json) record commands, mutations and source hashes; the [registration](../registrations/S01-selective-consuming-gate.md) fixes the comparison scope.

## What the work counts show

A stable selective witness contains eight left rows with distinct payload children and four matching right rows for only one child value. Three requests require 12 receipts. Direct execution performs 36 composite lookups and enumerates 12 successful pairs without retaining any pair. Retained execution constructs four pairs once and makes 12 retained visits. The two fresh metric runs agree exactly; counter-free runs report zero diagnostics and identical answers.

This is evidence of different work, not equivalent-duration operations. Retention buys an existing eligible relation but pays construction, storage and invalidation. Direct lookup avoids that relation's ownership costs. Dense consuming cases can create many retained pairs that are invalidated after relatively few applications; sparse updates may change only a few. These are the opposing regimes the lifecycle pilot must measure.

## Scope and next decision

These are checked lowerings of two exact source ASTs, not general generated matchers. Query admission requires a closed serial driver and distinct variable-to-ground binding instructions; failed or unsupported admission is not a logical failure. The certificate's source ordering is significant. The broader language has not adopted these restrictions.

T065 remains active for prospective lifecycle registration. Vary key fanout, structural selectivity, request frequency and update fraction independently, retaining consuming and propagation cases. Include the direct composite-index control, retained maintenance and admitted generic controls; charge preparation, setup, updates, observation and disposal. Measure whether fewer lookups repay the extra relation state before choosing any retained architecture. General subscriptions, partial joins and source-derived compiled access remain wider S01 questions.

Implementation and tests: [selective lowering](../../../research/chr-compiled/src/selective_join.rs), [independent source gate](../../../research/chr-compiled/tests/selective_join_gate.rs). The existing all-pairs experiment remains separate evidence with its original scope.
