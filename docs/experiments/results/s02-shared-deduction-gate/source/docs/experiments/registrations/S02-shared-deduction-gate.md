# Gate reuse of newly computed equality transitions

The [derivation breadth review](../results/S03-derivation-breadth-review.md) selects T072 before another local derivation refinement. Existing relational and contextual paths already support partial constructor access, incidence or direct matching, and isolated consumption. This gate tests a distinction they do not establish: reuse of an equality consequence computed after contexts fork.

## Operational comparison before implementation

| Organization | How the tested equality-enabled application becomes available | Distinction this gate can establish |
|---|---|---|
| Flat relations | Equality/constructor deductions update relations and matching accesses established facts; occurrences retain distinct resource identities. | Existing corrected control. Partial access and incidence indexing alone do not distinguish a new mechanism. |
| Contextual overlays | A context updates its equality parents/descriptors over shared immutable nodes, then matches and consumes local occurrences. | Existing complete-source control repeats deductions after a fork. |
| Shared equality transitions | An exact equality-state identity and equation select a previously derived successor state and child equations. The caller still owns its queue, failure and resource effects. | Newly derived equality states and decomposition work can be reused across contexts; no consumption is replayed. |
| Union-find expressed through CHR | Local find/link/descriptor rules perform the merge and activate source work in the same rule substrate. | Not implemented by this transition cache. Rule scheduling, administrative work and occurrence ownership require a distinct comparison. |
| Incidence/strategic port rewrites | Local graph edges expose affected identities, constructor ports and consuming rule sites. | Incidence alone exists in a control. Fusing administrative and source rewrites may differ; it remains untested here. |

Shared transitions are operationally a memo table for one equality-state transition. This overlaps S05's exact-state reuse question, but omits resource/history state because those do not determine a local equality step. It is not an equivalence to relevance-projected keys or an arbitrary whole-source continuation. The first gate deliberately tests the exact key before generalizing its validity.

## Candidate and invariants

Use arena-local monotonically fresh equality-state identities. Clones initially share an identity. Every newly computed nontrivial equality step receives a fresh successor identity; a cache hit adopts that recorded successor. Immutable constructor nodes cannot change. Keys include the current equality-state identity and ordered canonical input identities. Stores from different arenas never share the table.

A recorded transition owns the resulting parent/descriptor maps, the newly deduced child equations and its failure result. It excludes live occurrences, propagation history, caller's pending queue and output ownership. Applying it changes only equality state and appends exactly the child equations that the corresponding ordinary step would produce. Failure remains local. One source service step stays one step, so caching does not contract competing source schedules.

Retain at most 4,096 transitions per arena for this gate. Existing hits remain usable after the cap; misses continue ordinary equality work. This is a bounded experimental retention policy, not a language or optimal eviction choice. Discarding all arena owners must release the cache, with no ownership cycle.

## Discriminating witnesses

1. Fork before a shared constructor equation is solved. One sibling computes the deductions; the other must reach shared successor equality maps using the recorded transitions. Both must export the same independently derived values. This cannot pass by sharing only initial constructors.
2. Add different pending equations and separate local token claims. Reused equality must preserve both queues and independent consumption, including duplicate resources and later failure.
3. Start from different bindings or change an argument identity. An apparently similar request must not reuse an incompatible transition. Cover clashes and finite-tree cycles.
4. Run the existing generated equation-pair oracle under enabled and disabled sharing; compare complete exports and matches.
5. Run complete source programs with choices, consuming heads, fresh outputs, aliases, residuals, late failure and finite service against the independent scalar evaluator and the ordinary contextual/relational controls. Include a source whose equality state is actually shared after a source fork, rather than only low-level store calls.
6. Exercise the retention cap and confirm continued correct computation for misses and existing hits.

This is a semantic/mechanism gate. No comparative timing is registered. A passing gate justifies a prospective lifecycle comparison including high/low reuse, incompatible context updates, lookup/retention, changed queries and disposal. Broader local rewrites, source-derived lowering, generalized keys and sustained lifetime remain required regardless of this result.
