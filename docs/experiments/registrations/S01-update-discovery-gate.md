# S01 update discovery: prospective semantic and mechanism gate

This registers a correctness/mechanism gate, not a timing matrix. It tests whether an update-driven join really avoids revisiting unchanged combinations, while preserving consuming source behavior and propagation. Comparative cost registration follows only after the paths are credible and sizing is complete.

## Source and independent meaning

Use identified `left(Key, Value)` and `right(Key, Value)` occurrences and serial `request(Key, Round)` occurrences. A propagation rule with all three heads emits `receipt(Round, Left, Right)` for each ordered source tuple. A lower-priority request-consumption rule acknowledges the round only after propagation is exhausted. A driver posts the next request after acknowledgement. The complete endpoint retains receipts, row residuals and a done marker. This fixed committed policy is part of this gate, not a new language guarantee.

A mutation variant replaces one identified right occurrence between requests with a fresh occurrence. An equal-valued replacement must participate in the next round without inheriting the consumed occurrence's propagation identity. A late-binding variant posts rows with a shared unknown key and binds it between requests; constructor matching remains nonbinding before the equation.

An independent enumerative oracle maintains a plain list of source rows by identity and an explicit set of `(left identity, right identity, request identity)` fired tuples. It computes expected receipts from a Cartesian product at each round, independently of candidate indexes or retained matches. Full residuals and joint aliases are checked through the existing independent observer; candidate-specific identity numbers are mapped, not required to match allocation order.

## Competing paths

1. Existing Global/Active × Scan/Indexed source execution, with the fixed policy implemented only where supported. Unsupported policy combinations must be excluded explicitly, not silently run different source work.
2. Update-driven retained pairs indexed by key and occurrence incidence. A new request enumerates matching pairs; a row insertion/removal changes only its incident pairs. Binding updates invalidate all and only affected key memberships. Token lifetime follows the source identities.
3. A direct prepared join plan using current key buckets and fresh pair enumeration, without retained pairs. This separates compiled discovery from stored intermediate work.

The retained candidate must not walk every stored pair merely to ask whether an update affects it. Conversely, emitting each requested receipt is necessary work and cannot count as avoidable repeated discovery. Count newly constructed pairs, retained-pair visits, invalidations, candidate tests, token checks and emitted receipts separately.

## Frozen gate cases

Use N = 1, 2, 4, 8 rows per side; R = 1, 2, 4 requests; and key-group counts G = 1 or N. Round r requests key r modulo G. Row i has key i modulo G and a distinct ground value. Expected receipts are the matching left/right Cartesian products per request, preserving duplicates if source occurrences have equal values. No-join key requests and R = 0 are adverse controls.

For each N >= 2, additionally check: one right replacement with equal payload; one replacement with changed payload; one shared unknown becoming the requested key; duplicate equal-valued rows; two independent unknown payloads versus one shared payload; and request arrival before versus after rows under the declared policy. Cover a consumed partner variant separately with a hand-traced unique-partner source so nondeterministic consumer choice cannot be mistaken for oracle error.

All finite endpoints must exhaust and match complete expected observations and raw multiplicity. Adverse oracle mutations must detect missing/duplicate receipts, merged unknowns, stale consumed IDs and illegal early acknowledgement. A zero-budget call and repeated bounded resume must agree with an uninterrupted finite run. No trace or output is published from a cutoff as a completed result.

## Bounds, interpretation and subsequent registration

Gate bounds: 2,000,000 source/service steps and 60 seconds per process. Run semantic gates with counters enabled and disabled. Mechanism counters must disappear in disabled builds while behavior remains identical. Run at least two fresh diagnostic executions for the retained/direct/control comparison; record source hashes and exact work agreement. Gate time is not performance evidence.

If only sparse keyed cases favor maintenance, retain the dense/output-heavy cases for cost measurement. If output enumeration dominates, use phase attribution and add a selective structural condition before increasing sizes; register that source extension first. If current access already avoids rediscovery, this is contrary evidence rather than a reason to weaken the control. If invalidation is incorrect, fix the candidate before measuring costs.

After the gate, register a bounded native lifecycle pilot including cold preparation, changed queries, construction/maintenance, complete observation and disposal. Vary key fanout and update fraction independently; include insertion/removal and broad alias changes. Exact sizes and repetitions depend on noncomparative sizing and must be fixed before primary timing. This gate alone cannot rank architectures or close S01.
