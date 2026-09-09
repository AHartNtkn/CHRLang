# Next: distinguish equality progress from changes that invalidate matching

The [cursor lifecycle](S10-resumable-lifecycle.md) exposes a large penalty from invalidating on redundant equality: revised resumable traffic rises from 0.41 MB to 4.74 MB on the paired history source. This must be investigated before treating it as an architectural cost.

Compare two stronger controls. Runtime invalidation can distinguish processed work from actual changes to equality information or canonical identities. Source analysis can eliminate syntactically trivial self-equalities before they reach the runtime. Do not equate these mechanisms: dynamically redundant equations may be outside a syntactic control, while compile-time elimination avoids execution and checking altogether.

Inspect Store::step and its consumers before choosing the interface. Queue progress must remain observable for fair service and completion even when no matching information changes. A correction must preserve partial constructor deductions, canonical key changes, late guards, newly enabled earlier matches, failure and ongoing-source progress. Do not weaken the meaning of the existing progress return value to avoid invalidation.

Establish independently checked source witnesses for syntactic self-equality, dynamic already-entailed equality, a meaningful binding change and pending decomposition with intermediate information. Include effects and source ordering around the operation; source-side simplification must preserve complete observations and the accepted progress contract.

After the first executable gate, register paired lifecycle attribution if the mechanism changes consequential work. Charge analysis, tracking, invalidation and preparation; compare eager and resumable execution with Scan and the source-eliminated control. The objective is to establish necessary responsibilities, not to favor a retained cursor.

The breadth review prefers this bounded control because the measured conservative policy can reverse a whole-path traffic comparison. Support-aware conditional joining remains the strongest ready alternative and must be reconsidered after the gate. Other language, lifetime and complete architecture obligations remain active.
