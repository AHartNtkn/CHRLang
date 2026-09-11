# Reuse complete consuming callers

Test reuse of a completed branch-local caller, not substitution into a running
caller. The cache owns one prepared program and keys the complete ordered query:
constraints, variable identifiers, output names and output aliases. Equal visible
values alone are not a key. Returned Answer objects keep their existing local
variable namespaces; this experiment does not attach them to a live store.

Compare reuse on/off against Direct and independent scalar/reference execution.
Vary token multiplicity0/1/2, visible value a/b, extra caller facts, plain/aliased
outputs, fresh input identifiers, program priority, fresh output structure and
failed/duplicate alternatives. A repeated query must preserve full raw answers,
residual occurrences, within-answer fresh-variable aliases and derivation order.
Different program owners and changed contexts must not share cached results.

First validate this mechanism and bounded failure: a query exceeding its execution
bound must not install a partial cache entry. Keep full results owned and reusable
after individual delivery vectors are dropped or changed. Cancellation applies
to delivery of already completed caller answers; no claim is made about stepping
an unfinished caller. The existing ordered pipeline has a single-result caller,
so complete caller reuse does not force extra alternative work there.

After correctness, apply the same mechanism to projected and enumerated ordering,
including lazy enumeration. Keep non-reuse controls and Direct source execution.
Register the exact timing matrix after confirming the integration and preparation
ownership; retain all source-order checks. A full portfolio review follows this
package. Prioritize this because many successful hidden derivations invoke an
identical caller; broad source contexts and selective retention remain alternatives.

## Cost matrix registered after correctness

The576-context test and bounded-failure/owner test pass. Apply one cache per
prepared consuming caller, shared across its delivered assignments and queries.
Use the preceding64 lifecycle scenarios and seven modes: projected, enumeration,
lazy enumeration, their three memo variants, and full Direct source execution.
Thus448 cells,896 allocation repetitions and2240 ordinary runs, seed607109.
Compare each memo variant against its own non-memo path; additionally compare
memo projection against memo enumeration, memo lazy enumeration and Direct.
This gives384 comparisons under the same clock-floor and10%/all-five rule.

Every path keeps dynamic consumer buffers and complete lifecycle accounting from
S06-ordered-cost. Cache keys, cloned result delivery and final cache disposal are
charged. Preserve the same45-second wall/30-second CPU/1GiB bounds. Retention
extends across the eight-query session; this package does not choose a general
retention policy. The Direct source control runs its full source; it does not use
this per-assignment completed-caller cache. State that distinction in conclusions.
Run a full57-question portfolio review after interpreting the result.

## Known-one-answer repair

The initial matrix has one qualified reuse loss, at two coordinates, dense,
unpadded, one query, retained first answer. All16 one-query cancellation scenarios
request more bytes with unconditional caching: no second caller invocation exists.
When the session contract allows at most one answer, bypass cache lookup/insertion.
This uses an explicit consumer bound, not a predicted hit rate.

Validate all16 such scenarios with memo/non-memo versions of the three ordering
paths:96 cells,192 allocation and480 ordinary runs, seed607110. Require identical
requested and peak ownership for every repaired/control pair. Keep the initial
matrix as the unconditional-policy experiment and report this repair separately.
The generic completed-caller cache remains controlled by its explicit reuse flag.
