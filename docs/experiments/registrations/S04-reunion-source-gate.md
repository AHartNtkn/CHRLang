# S04: temporary separation and reunion source gate

Test whether independently evaluated private occurrences can be reunited into ordinary consuming CHR execution without reexecuting their private work. This is a source/correctness experiment, not a timing comparison or a general independence theorem.

## Candidate and contract

Check a source prefix of local rules followed by joining rules. Every head has a literal atomic ownership key in its first argument. Each local rule's heads have one owner and every emitted constraint preserves it. Initial components have disjoint variables. Joining rules may connect owners through bindings, consume resources and enable local rules again. Rules and ordinary priority remain unchanged after reunion. Same-predicate occurrences with different owners must separate.

The experimental entry explicitly checks the prefix and ownership premise; inference for arbitrary sources is not claimed. Local evaluation uses the current source transition implementation, retaining complete bindings, live occurrences, propagation history and identity supplies. At quiescence, take the Cartesian product of local alternatives, rename private fresh identities, relocate occurrence/history identities and resume the full ruleset. Do not merge outputs and call them a reunited state. Raw multiplicity must survive independently identical alternatives.

The gate's observation contract is complete finite raw answers up to renaming and permutation. It does not claim equal public service traces or first-answer latency. A finite source-step budget must return an explicit error for unfinished local or resumed computation, never exhaustion or partial success. All states are query-owned; cancellation by dropping a running public engine and ongoing-sibling scheduling need a later resumable entry.

## Discriminating tests and controls

Compare every accepted source with the independent scalar evaluator and ordinary Copy execution. Exercise two same-predicate private components, independent choices and substantive local recursion; a joining rule binds their outputs or consumes a shared resource; local work can reactivate after that binding. Include zero-work/frequent joining, failed choices, duplicate choices, fresh residual aliases, propagation history and competing consumers under fixed owner heads. Compare a pure no-binding product as a witness that permanent independence cannot implement the binding/consumption join. Reject initial shared variables, unknown ownership and local owner escape.

Observe a failing test before implementation, then complete semantic tests, existing restoration regressions and strict Clippy. Bound the exhaustive gate to small depths/branching and at most 100,000 source steps per run; include an intentionally ongoing source under a much smaller limit. Count local source steps, product states and resumed steps separately. Require a witness where private work is done fewer times than ordinary coupled execution, independently of answer equality. No timing or allocator claim is permitted.

## Interpretation and next action

A passing gate establishes actual reunion only for the checked phase/ownership fragment. Attribute counterexamples to incorrect identity transport, history/resource handling, eligibility or observation assumptions and repair consequential defects. A proof sketch must state why local steps commute and why later rule competition is preserved; finite tests alone do not prove general source correspondence.

After the gate, reassess complete cost measurement against T072 integrated dependency repair. Before a cost comparison, provide reusable preparation, bounded service/cancellation, allocation ownership and source-derived acceptance accounting. Dynamic mid-computation split/reunion, repeated cycles and broader ownership inference remain required even if this first phase gate passes.

## Progress extension during the correctness gate

The finite batch gate passes its initial witnesses but waiting for every local alternative is insufficient for an ongoing source with a finite sibling. Extend the same checked phase to resumable service before calling the gate complete. This addendum records the extension during correctness development, following its failing progress test; it is not a prospective cost registration. No comparative timing has run.

Alternate local branch steps, one lazy product materialization and resumed branch steps. A newly completed private alternative combines with already available alternatives from every other component. Freeze their current index limits in a cursor so each combination appears exactly once, including equal-valued alternatives. Retain incomplete local branches and service all three queues fairly. Preserve full finite raw answers and demonstrate a finite sibling answer within 1,000 services against ordinary source execution. Continue to distinguish service fairness from a constant wall-time quantum or identical event order.

Add two-to-four-component product tests and weak-owner cancellation checks. A finite completion helper has an explicit total service budget including product work; reaching it returns an error without partial success. Reusable checked preparation, retained-cache costs and repeated dynamic separation remain subsequent requirements.
