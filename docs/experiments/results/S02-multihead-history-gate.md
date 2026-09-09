# Direct equality graphs can preserve general head and history semantics

The local equality graph now executes deterministic multi-head rules with kept and removed occurrences, propagation history and competing consumers. Forty-seven finite source configurations agree with independent scalar evaluation and conventional scanned and indexed execution. This establishes broader semantic coverage; selective activation and comparative costs remain unmeasured.

## What changed

The earlier local plan accepted one `take` and one `token`. The new source-driven executor accepts arbitrary nonempty combinations of kept and removed heads, nested constructor patterns, repeated variables, and deterministic bodies that post constraints, equate terms or fail. Guarded rules and alternatives are explicitly rejected in this gate.

Head matching reads the existing graph's handles and constructor descriptors directly. It does not serialize an answer or construct a substitution store to drive execution. Captured variables remain handles, and fresh body variables receive fresh handles. Equality and finite-tree failure use the same local rewrite implementation as the earlier integrated experiment.

Occurrence identity is separate from value equality. Head tuples contain distinct live occurrence IDs, and history is keyed by the rule and ordered tuple. Removed occurrences disappear before the body executes; kept occurrences retain identity. The whole body completes, including its equations, before another rule is selected.

## Evidence that distinguishes the responsibilities

| Check | Configurations | What it establishes |
|---|---:|---|
| Duplicate values, late aliases, nested heads and competing rule order | 24 | Equality can enable new tuples without changing occurrence identities; source priority still selects the consumer. Late cases withhold the permit until equality completes. |
| Body effects, fresh aliases, contradiction and occurs failure | 4 | Effects share fresh variables correctly, and inconsistent bodies produce no answer. |
| Mixed kept/removed heads over reused prepared rules | 12 | One occurrence cannot fill two heads. Equal-valued duplicates are consumed separately, leaving exactly one kept item. Changed query variables do not leak between runs. |
| Duplicate kept occurrences | 4 | With n equal-valued occurrences, the ordered two-head propagation rule produces n(n−1) results. A history key based on values would lose valid firings. |
| Body barrier and distinct occurrence order | 3 | A permit posted later in a body enables the higher-priority consumer before selection. Reversing two distinct initial values changes which occurrence wins. |

Every configuration compares complete raw answers against all three controls. The reused prepared object additionally executes the 12 mixed-head queries. Three rejection checks cover body alternatives, guards and empty heads. All finite executions finish within the 200,000-call service bound; a cutoff would remain unfinished evidence.

The [source tests](../../../research/chr-relational/tests/multihead.rs), [executor](../../../research/chr-relational/tests/support/local_multihead.rs), [registration](../registrations/S02-multihead-history-gate.md) and [validation logs](s02-multihead-history-gate/source.log) record the scope. The reference interpreter and conventional controls are unchanged.

## What this does not establish

The executor scans candidate head tuples at every source boundary. It is a general ordered selection control over direct graph state, not evidence for selective wakeups or a faster integrated architecture. Candidate-environment copying, tuple enumeration and retained history are explicit current costs. The existing single-request subscription machinery does not automatically solve general tuple dependencies.

This gate also does not resolve guarded matching, disjunctive search ownership, general contextual sharing, arbitrary source lowering, retained-state lifetime or total efficiency. Its deterministic source support is broader than the earlier take/token fragment, but is not a complete CHR architecture.

## Next comparison: selective tuple activation

Keep T072 active for a bounded comparison of scanning and selectively activated head tuples over the same equality graph. A cached tuple must track actual inspected equality/constructor dependencies, survive handle merges, reject consumed occurrences, preserve ordered history and reconsider new facts. Sparse equality updates and broad merges must both be included, with unchanged complete answers and body barriers. Work counts should distinguish fewer inspected tuples from extra registration and repair work before measuring cost.

This can change the integrated architecture decision by showing whether equality and general matching avoid repeated work together. It is more informative now than timing the scanning executor alone. Resource-aware source lowering remains the strongest distinct alternative, while adaptive reunion retains its measured favorable/adverse regimes. Reconsider those after the tuple-activation gate; their lower current priority does not resolve them.

The full relational test targets and scoped strict Clippy pass; dependency build-script warnings are retained in the logs. This package provides semantic evidence only. The research goal and all broader sequence obligations remain active.
