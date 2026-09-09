# Non-overlap must cover reads and effects, not only consumption

Disjoint consumed occurrences do not establish independent applications: one application can consume a head another keeps. Even disjoint head predicates can communicate through equality and require guard reactivation. The new source witnesses establish these limits without rejecting stronger non-overlap or effect certificates.

## Three discriminating witnesses

| Question | Source and explicit expected result | Consequence |
|---|---|---|
| Are disjoint consumed sets sufficient? | `p \\ q <=> mark` competes with `p <=> true`. The first consumes q, the second p. With the reader first, the result is mark; with the consumer first, q remains. | The consumed sets are disjoint, but consumption intersects a kept head. A conflict certificate must account for read/write interference. |
| Does non-overlapping consumption eliminate activation? | `p(X) <=> X=a | mark`, alongside `bind(X) <=> X=a`. With unknown X and no binder, p remains. With the aliased binder, mark results in either rule order. | Head predicates cannot share occurrence IDs, yet a body's equality changes a guard's eligibility. Conflict freedom over occurrences does not eliminate equality dependencies or wakes. |
| Can independent consumption actually commute? | `p(X) <=> left(X)` and `q(Y) <=> right(Y)`, with ground inputs and inert output predicates. Reverse rule order for zero through three duplicate pairs. | All eight configurations give the same explicit residual multiset. This is a favorable case for independence, not a general theorem about arbitrary bodies. |

The contextual match enumeration independently confirms that the first witness has one application of each rule and disjoint consumed IDs. Complete answers are checked against hand-written expectations, the owned-syntax scalar evaluator, Scan execution, eager contextual execution, resumable contextual execution and conditional execution. Inferred unary dispatch is also checked where enabled. The 13 new full-source configurations run in each of two builds: diagnostics enabled and counter-free, both with serial accounting, unary dispatch, prefix joins and equality invalidation. All ten tests in the expanded file pass in each build; strict scoped Clippy and package formatting pass. [Logs](s07-overlap-obligations/) record the gate.

The [existing head-property evidence](S07-head-property-gate.md) additionally covers constructor discrimination, competing unary rules, multiple instances of one rule, partial matching environments, propagation multiplicity and linked reformulations. These results are distinct: neither head count nor a single named rule establishes unique application ownership.

## Which responsibilities are actually candidates for elimination?

| Responsibility in the current conditional engine | Current evidence | What a stronger proposal still owes |
|---|---|---|
| Busy-body Boolean accounting | The [serial scheduler specialization](S07-serial-body-allocation.md) saves work without restricting source programs. | A certificate cannot claim that saving as a benefit over the strengthened unrestricted control. |
| Propagation history | Consuming rules already bypass this history; pure propagation still needs once-per-tuple behavior. | Show an additional history obligation disappearing, rather than relabeling the existing consuming path. |
| Live occurrence supports and consumption | The kept-head witness makes lost eligibility observable. Alternatives also have distinct live scopes. | Prove the relevant read/write and contextual ownership conditions, and preserve residual multiplicity and cancellation. |
| Matching, guard dependencies and reactivation | Disjoint predicates still share equality effects in the binder witness. | Analyze body effects and aliases, or retain these operations. A pattern-only certificate is insufficient. |
| Discovery and source ordering | Unary inference and immutable constructor filtering already avoid some discovery. The kept-head witness requires the agreed rule competition. | Identify a further removable operation against these controls and preserve source effects and progress. |

This is a correctness and responsibility gate. It measures no speed, allocation or admission cost. There is no new property checker, declaration interface or mandatory restriction in this package. Stronger effect/ownership analysis could still remove coordination, permit batching or enable a simpler representation; those are untested beneficiaries, not rejected designs.

## Selection after the gate

Five bounded packages have followed the join breadth review: head-property sources, unary dispatch, unary lifecycle, serial accounting and this overlap obligation gate. The next package returns to sustained observation and retention under T074/S08. Existing equivalent complete answers have different owned footprints, and repeated-query lifetime can affect both total efficiency and architecture boundaries. The current output APIs, allocation meter and independent observations make that comparison ready.

Another non-overlap implementation would first need a sound effect/ownership property and a named additional runtime beneficiary. That remains consequential work, but the present witnesses prevent attributing existing serial savings or ignoring equality effects. No observed result currently justifies imposing a restriction. Sustained lifetime offers a more direct complete-architecture comparison at the next bounded cost.

T079 remains unfinished. Resume with a property covering kept/removed interference, shared equality effects and linked extensions; compare inference, checked promises and required admission only once an executable beneficiary is identified. Preserve favorable independent cases and quantify conservative false negatives. Reconsider this against the next S08 source/ownership result. Broader modes, finite domains, progress and semantic contracts also remain required.
