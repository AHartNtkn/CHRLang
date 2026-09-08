# Pure-carrier eligibility gate (T053)

This gate asks whether valid contextual lowering can avoid repeated source
selection and intermediate occurrence maintenance in an equation-free countdown.
The question matters because the current positive conditional case contains that
work. The [arena comparison](R03-arena-ownership.md) has a measured useful region
and adverse controls; another storage refinement is less likely to change the
architecture decision than establishing this distinct eligibility boundary.

## Source claim

For a carrier predicate, every head use must belong to exactly two single-removed
rules with no kept heads. One unguarded rule replaces a unary control constructor
with its child and preserves pairwise distinct opaque parameter slots in their
original positions. The child slot must also be distinct from every opaque slot. Its body is exactly that one repost: no equations, extra
goals, choice or fresh locals. The terminal rule has a distinct nullary control
constructor (constructor identity includes name and arity, so the names may
coincide); its body is not part of the contraction. Names and control argument
positions are derived from source syntax.

At an ordinarily selected Global step, there must be exactly one live carrier
occurrence. The remaining control must dereference to a finite ground spine
ending at the certified terminal constructor. Only this interval may contract.
Exit posts the terminal carrier and returns to ordinary arbitration before
executing its terminal body. Admission is checked again on subsequent entries.
This is a sufficient experimental boundary, not a language restriction or claim
that all valid contractions have this shape.

## Correspondence argument and adverse cases

At entry, unrelated higher-priority rules are disabled. An interior carrier step
changes none of their occurrences or bindings, and the certificate excludes all
external carrier observers/consumers. Lower-priority rules cannot preempt the
remaining recursive step. Therefore those interior source applications remain
selected until the terminal control. Opaque aliases remain unchanged; they are
not solved to make a head eligible.

Single-head syntax alone is insufficient. With two live carriers, fresh repost
IDs rotate occurrence order and a different terminal effect may run first.
A repeated control variable is also unsafe: `p(s(X),X) -> p(X,X)`
can match `p(s(s(z)),s(z))` once and then block. External propagation can observe
intermediate carrier occurrences. Unknown tails
must not be mistaken for ground controls or consumed by invented base cases.
Terminal equality, failure and choice must remain outside the contracted interval.
Other rules may create carriers later, requiring renewed admission.

Reserve exactly the IDs that ordinary reposting would consume, and preserve the
terminal occurrence's ID. Expanded logical traces must replay with independent
source syntax. Per-application audit records must describe real corresponding
source transitions; a combined transition cannot masquerade as one ordinary
application. Diagnostic expansion costs belong to the selected diagnostic mode.

## Service contract

The existing [compiled search gate](R03-compiled-search-semantic-gate.md) requires
complete raw multisets, exact branch traces/identities and finite sibling service.
It compares generated/generic and access configurations whose administrative tick
counts differ. It explicitly makes no constant-time step claim. Exact wall time,
completion tick or inter-branch answer order is not inferred as an additional
semantic requirement here.

Spine inspection must nevertheless be resumable so a long finite input does not
monopolize branch service. While validating, the existing source occurrence can
remain present and the engine can stutter; no terminal answer is published.
Cancellation must dispose of partial validation state. Unknown or malformed tails
retain ordinary nonbinding source behavior. Any unsuccessful-admission overhead
must be identified before lifecycle conclusions; it is not free because the
optimization did not apply.

## Evidence required before measurement

Demonstrate actual avoided selection/store work, not only an inferred plan or the
unchanged engine passing examples. Use independent source replay and analytic
answers, renamed predicates/constructors/control positions, opaque sharing,
terminal effects, exact virtual IDs, dynamic multiple-carrier rejection, external
observers, guards/additional goals, unknown controls, renewed admission, cancellation
and a finite sibling beside divergence. Verify trace/audit handling separately
from counter-free execution. Preserve the independent reference.

A successful gate can justify a prospective complete-path comparison against the
current competent explicit control and conditional execution. It does not itself
show that contraction beats sharing, eliminate all opaque work or authorize
arbitrary eager relation execution. If this boundary cannot preserve the selected
semantics, record the rejection and reassess its value instead of broadening it
until an easier claim passes.
