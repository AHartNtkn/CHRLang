# E06a: finite-tree unification in actual net rules

The finite controller now supplies dereferencing, occurs traversal and transactional
unification, using 207 active-pair rules over 58 agent types. All 864 reference
cross-checks pass and replay exactly, and the expanded 13-test suite passes. This
establishes a concrete service gate; it does not establish a full CHR net engine
or an efficiency advantage.

## Evidence and reproduction

```
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-nets -v
cargo run -q -p chr-cases --example net_unification_cases > docs/experiments/results/E06-unification-reference.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-nets/check_unification.py docs/experiments/results/E06-unification-reference.jsonl docs/experiments/results/E06-unification-gate.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-nets/check_unification.py docs/experiments/results/E06-unification-reference.jsonl docs/experiments/results/E06-unification-replay.jsonl
```

The reference exporter runs 432 requests: 12 terms squared under three initial
binding environments. The net runs each in FIFO and newest-first order. All three
original variables are selected, so joint alpha-normalized projection checks
unbound-variable relationships as well as constructor values. No new logical
variables are introduced by this service. There are 328 successful net runs and
536 failures, corresponding to 164 successful and 268 failed reference requests.
The largest observed request uses 378 net interactions, below the registered
100,000 bound. Every output includes an unchanged original table; failure exposes
no private partial substitution. All outcome/work/storage counters replay exactly.

The reference implementation imports no candidate code. The new research example
constructs equations using the existing syntax and runs the reference independently.
Candidate decoding projects the resulting table and compares complete selected
outputs, including shared holes. The exporter requires reference exhaustion and
empty residuals for these equality-only requests. The original reference registry
and lambda tests pass, as do Clippy for chr-cases and workspace formatting.

Hand-derived tests separately cover direct and indirect occurs failure, alias
chains, both constructor-arity mismatch directions, repeated-variable clashes,
variable/constructor symmetry, and failure after an earlier private binding.
Occurs has 128 checks against a separate structural oracle, and dereference checks
preserve the table and leave constructor children unresolved at the root service
boundary. A deliberately cyclic input alias table remains unfinished and cannot
publish a result; valid service inputs require an acyclic binding graph.

## Service correspondence

`Deref` inspects Ref/App. App returns its root unchanged with the table; Ref calls
preserving lookup and follows a bound value until it finds an unbound representative
or App. This is finite on a finite acyclic input table. It does not deep-normalize
constructor children as part of root dereferencing.

`Occurs` maintains an explicit list of pending terms. Each term is dereferenced;
an unbound Ref is compared with the target handle, and an App adds its children
to the pending list through finite Append rules. A matching representative returns
true; exhausting the list returns false. The table is unchanged. Shared subgraphs
may be traversed repeatedly: no visited-set or cycle-check-elision optimization
is assumed. The target is a currently unbound representative when called by UBind.

`U` maintains a private table and an equation list. Both roots are dereferenced
before dispatch. Equal free representatives add no binding; distinct free
representatives may be aliased. A free representative and constructor require a
successful occurs check before binding. Equal constructor tags are decomposed
only when their child lists have equal length; mismatches fail. Zip accumulates
child equations in reverse order, a permitted internal order for this atomic
finite-tree service. It is not a CHR rule-order change.

Each successful transformation preserves the equation solutions under the private
table. A new binding names a previously free representative, and occurs establishes
that it cannot introduce a cycle. Alias binding joins distinct unbound roots.
Constructor decomposition preserves conjunction, while clashes and occurs failure
have no finite-tree solution. With finite acyclic inputs, root lookup and occurs
are finite, and the equation-worklist procedure either eliminates variables,
decomposes constructors or fails. No branch mutation is published between these
steps. Success returns the resulting table; failure erases private state and returns
None while the caller's original table remains preserved.

All these operations are generated finite RHS graphs. Named wires in
`unification_rules.py` are compile-time interface notation; they are not runtime
unification variables. Rule validation still requires every port exactly once.
No host unifier, table scan or recursive occurs check is hidden inside a net step.
The ordinary queue can yield between any two active-pair interactions.

## Costs and open work

This gate does not compare runtime speed. It retains explicit unary IDs, table
lookup, copying of operands needed by both checking and binding, table preservation,
and generic graph/slot bookkeeping. Register direct map/table controls and whole
service construction/extraction costs before comparative measurements. Preserve
negative and repeated-variable workloads, not just successful fresh bindings.

Wake-up effects, conditional branch support, CHR occurrence/history integration,
and global/regional source certificates remain open. The current contract processes
a frozen private service request; it does not permit concurrent mutation of the
same table. Distributed fusion and multiport protocols need their own atomicity
and alias tests. E06b's general encoded source machine and E06c's native-label
correspondence continue independently. No source restriction or production
architecture is adopted by this gate.
