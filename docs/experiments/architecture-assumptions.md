# Architectural assumptions to test

This register identifies implementation choices that must not silently become architectural requirements. The [sequence](sequence.md) governs selection and comparison; the [question map](coverage.md) owns priorities and dispositions. An entry here is not an instruction to implement another prototype.

| Assumption to challenge | Evidence and its boundary | Architectural contrast | Question / stage |
|---|---|---|---|
| A1. Constructors and equality require a privileged service | [E18](results/E18-relational-gate.md) establishes a finite integrated monotone fragment; [E15](results/E15-costs.md) compares complete equality requests | Dedicated representation versus integrated constructor/identity relations or local graph execution, including application effects | Q2; R00/R02 |
| A2. The unit of computation must be a complete source step or state | E04 shares expansion construction; E09/E15 share whole-state jobs | Generated operations, incremental dependencies or local rewrites that avoid complete-state interfaces | Q1/Q2/Q3; R01–R03 |
| A3. Matching is a repeated request to enumerate tuples | [Maintained sizing](results/E09-maintained-sizing.md) still repeatedly discovers cached extensions | Compiled access plans, changed-occurrence activation, maintained joins and cheap recomputation | Q1; R01 |
| A4. Compilation means unfolding syntax or encoding interpreter microsteps | [E13](results/E13-costs.md) tests single-entry unfolding; [E11](results/E11-matched.md) tests bounded traces | Generated rule execution and direct relational/derivation compilation | Q1/Q4; R01/R04 |
| A5. Source consumption and propagation need central per-branch tables | Existing protocols validate their own occurrence/history machinery | Ownership, supported activations or local effects with a legal firing/consumption argument | Q2/Q7; R02/R05 |
| A6. Observation requires constructing every complete physical answer first | [E14](results/E14-graph-costs.md) separates clone, comparison and export effects with mixed costs | Exact graph/stream observation versus competent eager delivery, including ownership and retention | Q9; R05/R06 when consequential |
| A7. Search requires physical complete branch snapshots and raw-lineage enumeration | Current diagnostic counters expose those objects; factoring and conditional probes already use other organizations | Explicit state versus symbolic/support/demand-driven execution; retain required observations without prescribing diagnostic objects | Q3; R03 |
| A8. Completion, failure and identity lifetime share one global mechanism | E06 gives off-output failure/label hazards; E09 gives finite-service protocols | Independent choices for validity, completion, work suppression and reclamation | Q7/Q10; R02/R03/R05 |
| A9. A competent scalar control is already a competent compiled architecture | Persistent execution still selects rules generically; specialization has a narrow entry representation | Generated and interpreted paths with comparable indexes, plus activation and static-information contrasts | Q1; R00/R01 |
| A10. Local component gains must precede integrated architectural experiments | Component results often retain common service boundaries | A minimal complete path that exercises an eliminated boundary, with ablations where meaningful | Q2/Q8; R02/R05 |
| A11. Named examples define the evaluation domain | Existing cases cover useful behaviors, not a workload distribution | Select by computational contrasts, including ordinary no-OR execution; present regime-dependent tradeoffs | Q11; all selected experiments |

A8 separates identity lifetime, publication/completion and normalization of unfinished obligations. Evidence for one does not settle another. In particular, invalidating an interpretation does not require immediately reclaiming all affected graph nodes, and a completed-looking output does not discharge active off-output work.

All these assumptions have unresolved architectural consequences. They enter R00 screening; no fixed A-number ordering grants priority. A known component tradeoff can be adequate evidence until a selected architecture makes its uncertainty consequential.

For integrated constructor representations, the finite-tree comparison must distinguish projected cycles from cycles in the union of incompatible alternatives. Unknown head arguments must not acquire structure solely to match a rule. Descriptor/value columns, occurrence identities, alias updates and residual extraction need an explicit interpretation. These are semantic obligations, not requirements to imitate a transactional unifier.

For compiled/local alternatives, the correspondence may use constraint denotation, legal event ordering or another justified account of unfinished state. A scalar microstep API is not mandatory. Experimental restrictions remain explicit; the source contract and implementation boundaries are different objects.
