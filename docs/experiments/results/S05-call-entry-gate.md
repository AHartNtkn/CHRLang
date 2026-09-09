# A complete private phase is a checked call-reuse boundary

The call table now checks an initial query boundary that supports shared caller variables without moving work across source-rule priorities. Selecting a call's first rule is insufficient, and variable disjointness alone does not preserve source progress. Both failures have executable counterexamples.

## The two insufficient arguments

**A selected first rule does not make a recursive call atomic.** In the first counterexample, the ordinary source initially selects a recursive call step. A supply rule then becomes the next applicable rule and binds its input before the base case. Ordinary execution returns `known`; isolated completion of the selected call returns `unknown`. The independent trace verifies that the first selected rule really belongs to the call.

**Disjoint variables do not imply the same progress behavior.** An earlier-priority `spin() <=> spin()` prevents the ordinary machine from reaching an independent failing call. A 1,000-step execution remains unfinished; the rule's repeating form explains why this policy continues indefinitely. Isolated call expansion immediately discovers failure. The initial disjointness-only admission accepted this case; the [failing gate](s05-call-entry-gate/progress-counterexample.log) preserves that counterexample. This is a scheduling/progress distinction, not evidence that the failing call has a successful answer.

The general operation API therefore requires a complete execution-phase or commutation argument. Its documentation no longer treats selection of the first rule as sufficient. Different source policies remain possible architectural studies, but cannot silently enter a policy-matched control.

## What the checked entry establishes

The [checked query entry](../../../research/chr-reuse/src/calls.rs) requires all of the following:

- The prepared private family's rules are the exact rule-priority prefix of the caller program, in the same order.
- Every caller rule touching a private-family head belongs to that prepared family. An external consuming or kept head cannot observe or take a private occurrence.
- The initial query contains exactly one private-family occurrence, and the selected index identifies it.
- The private computation completes every alternative with no residual work within its service bound. Suspension and cutoff produce errors, not cached success.

The family's existing single-head and closed-body checks remain in force. These conditions establish a complete initial phase in each accepted branch: the ordinary machine services applicable private work before outside rules; generated calls remain inside the family; no second initial family occurrence competes with it. When isolated evaluation has no residual private work, the corresponding phase has finished.

Caller variables may be shared with the call. Their current relationships are present in the initial query, and every interface binding is replayed before caller execution resumes. The allocator covers all caller variables, not only those mentioned in the key. This is more useful than a blanket variable-disjointness rule: an observer or later supplier can share the parameter and receive the resulting bindings.

This API accepts initial query syntax, not arbitrary live cursors with pending effects or hidden bindings. It returns per-alternative interface equations; the caller must consume the selected occurrence, apply those equations and continue under the complete caller program. The gate demonstrates this through ordinary source execution. Automatic mid-continuation extraction remains unimplemented.

## Complete source checks

| Test | Evidence |
|---|---|
| Priority or ownership interference | Rejects preceding caller rules, an external multihead consumer of the private predicate, and multiple initial family occurrences. Rejection happens before looking up a cached answer. |
| Independent choice products | 24 configurations vary call depth, initial constraint order, an outside failing alternative and variable offsets. Complete output/residual multisets and raw multiplicity match independent source evaluation. |
| Shared observer | Accepts a caller observation using the call parameter and preserves the full joint aliases. Shared-variable syntax alone is not treated as an effect. |
| Later shared updates | Two callers supply different input constants after the phase; a third conflicts with its result. One call computation and two hits preserve independently correct success or failure after resumption. |
| Caller starts another private phase | After replay, an outside rule posts another private call. Resuming under the full program produces all four expected raw alternatives with independent fresh results. |
| First-rule and progress counterexamples | The checked entry rejects both; the unrestricted operation is still tested as a countermodel to those insufficient admission arguments. |

The [source tests](../../../research/chr-reuse/tests/call_reuse.rs) retain the earlier 216 key/transport configurations and the whole-state versus call-level reuse witness. Expectations use independent recursive substitution and ordinary source rules. No reference-interpreter implementation was changed.

## Limits that affect the next cost comparison

The accepted private phase is an experimental source property, not a proposed mandatory language restriction. Some safe calls outside a rule-priority prefix will fail this checker. Multiple independent private occurrences, resumable private work and general resource/history transport remain separate opportunities; their rejection here is not an architectural disposition.

The table currently evaluates all private alternatives before returning caller work. Accepted computations are bounded and complete, but batching can delay first caller observation compared with interleaved branch service. Complete raw answers do not establish equal latency or identical diagnostic event order. The lifecycle comparison must measure this directly.

Admission structurally checks the caller program and query at each entry. Preparing that ownership/order evidence once is a possible implementation improvement, not a measured benefit. Full costs must include admission, argument keys, isolated execution, fresh transport, caller continuation, observation, cancellation and disposal. Native compilation and any caller preparation cannot be silently omitted.

## Validation and next work

The final call target passes 13 tests in both metrics-enabled and counter-free builds, and strict scoped Clippy passes. The full reuse package passes 67 tests; the strengthened final call target also passes. The runs are recorded in [the receipts](s05-call-entry-gate/). Commands are `cargo test -p chr-reuse`, `cargo test -p chr-reuse --no-default-features --test call_reuse`, and `cargo clippy -p chr-reuse --test call_reuse --no-deps -- -D warnings`.

T075 remains active. A bounded lifecycle comparison is now justified for this checked phase, provided it includes the complete resumed caller and credible direct, whole-state, compiled and applicable source-specialized controls. Establish admission/preparation ownership and cancellation before prospective timing registration. Reconsider broader caller extraction and the strongest ready S02 dependency repair at that boundary; structural solving and restoration/reconnection remain required distinct alternatives.

This is the second package since the integrated breadth review. The architecture goal remains active; no total-efficiency or language-adoption conclusion follows from this gate.
