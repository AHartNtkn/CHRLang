# A6 actual completion gate

Status: registered before matrix execution. The synthetic graph gate passes4,290
comparisons and replay; it is not a substitute for source integration.

Run all64 E00 registry cases in registry order, each in a fresh test-binary process,
then exact replay:128 children. Advance paired eager and borrowed-completion
Machine instances with identical FIFO frontiers, checking each event kind. This
FIFO policy is an experimental control. Use each case's existing source-step budget
and recognized-answer limit; do not stop at a raw completion limit.

At every completion, compare the borrowed snapshot against previously retained
snapshots using the graph comparator. Export it separately for validation and
require exact equality with that eager raw answer. Compare duplicate decisions
with the existing eager exact observer. Check recognized full observations against
hand-derived registry answers, raw multiplicity and exhaustion against the case
contract. Re-export retained snapshots after all subsequent source steps and require
unchanged answers. The independent synthetic oracle supplies the separate graph
comparison algorithm check; eager event agreement alone is not a semantic proof.

All source counter fields must agree after excluding explicitly measured eager
export dereferences and storage visits. Capture, graph comparison and validation
export counts remain separate; the external driver records actual queue peak.
This validation exporter deliberately exports every raw completion. Its work is
not a candidate cost result or a claim to avoid all duplicate exports in this gate.

Freeze reference source, registry/program fixtures, both candidate crates, syntax,
Cargo configuration, registration and harness before the first test child. Build
in release mode before recording the semantic matrix, recording compiler identity
and binary hash. Each child has30seconds and1GiB address space. Preserve full
errors and stop an unsuccessful version for investigation. Exact semantic/source/
observation counters must replay; elapsed time is only a bound diagnostic.

Passing establishes bounded completion/export/dedup correspondence for the existing
registry, including its intentionally incomplete answer prefixes. It does not close
A6 retention/traversal/cost questions, establish general fairness or expand the
registry's current synthesis scope. Costs need their separate three-control
lifecycle protocol with actual delivery export only for recognized new answers.
