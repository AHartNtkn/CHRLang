# Architectural assumption register

This register complements the direction audit. It distinguishes source obligations
from shared implementation choices and gives each relevant architectural contrast
an independent investigation entry. All entries below remain open. Mentioning an
alternative, implementing several engines with the same boundary, or validating
one interface does not investigate whether the boundary should exist.

The current omission is supported by the document structure: E06 compared richer
graph targets through fixed operation contracts; T015's relation graph and E15/E16
interfaces retained transactional equality; E11 encoded evaluator microsteps.
Those are legitimate bounded controls. The inference that the reference's internal
decomposition migrated into portfolio assumptions is a causal hypothesis, not
privileged evidence about reasoning. The sequence now makes the distinction
explicit and uses this register before extending mature comparisons.

## Assumptions, contrasts and evidence

| ID / shared assumption | Existing evidence and its boundary | Distinguishing investigation / assignment | Dependency and current disposition |
|---|---|---|---|
| A1. Constructors and unification form a privileged subsystem | [T015 graph](../goals/chr-sharing/notes/T015-relation-graph-construction.md) keeps immutable constructors and private complete equations; [E06 richer targets](results/E06-directions-initial.md) keeps a transactional request; E15/E16 exercise that interface. [T011](../goals/chr-sharing/notes/T011-conditional-kernel.md) already supports elementary conditional equations, so whole-term reuse is not the strongest existing alternative. | E18/E06: constructor relations over identities, consistency rules and conditional fusion integrated with program rewriting; compare the same representation behind a service boundary, a fine-grained shared worklist, and the competent arena control. | [First finite gate](results/E18-relational-gate.md) passes432recipes/replay and16tests. [Local substitution-stable eligibility](results/E18-source-interleaving.md) permits partial-equality/source interleaving in the current positive fragment. Full source effects, efficient classes and partial-sharing controls remain feasible; no broker/parallel dependency. Open. |
| A2. A complete source transition/state is the unit of execution and sharing | [E04](results/E04.md) shares expansion construction while still committing projected applications; E09 groups whole states. | E03/E04: supported dependency/event graphs sharing matching, body construction and effects separately. Probe two consumers with partially overlapping demand, conditional consumption, fresh locals and failed support. | Requires occurrence/effect correspondence, not a successful unifier service. Open independent entry. |
| A3. Matching enumerates complete tuples through a request | E09 scan/prefix/guard probes improve transient enumeration; [T012](../goals/chr-sharing/notes/T012-matching-and-wakeup.md) analyzes maintained subscriptions. | E03/E09: persistent partial-join relations with delta updates on introduction, consumption and aliasing. Compare full scan and current prefix controls, charging retained intermediates and invalidation. | [Integrated finite source gate](results/E09-maintained-gate.md) passes80 children under T019, independently of A1; larger sizing and real costs remain open. [Matching alternatives](results/A3-matching-directions.md) add seed-ordered recomputation, lazy search, multiway/hybrid joins and sensitivity traces. All remain open. |
| A4. Symbolic compilation must encode interpreter states and microsteps | [E11 registration](registrations/E11.md) encodes pending/substitution vectors, first-enabled selection and Robinson steps. | E11/E13: directly compile bounded derivations or candidate relations for a certified fragment, against trace encoding. Test addition and SK typing, nonground answers, explicit-choice duplication and rejected resource-sensitive inputs. | Needs an explicit fragment/correspondence, not a faster trace solver. Experimental restriction does not imply adoption. Open. |
| A5. Histories and consumption require per-alternative tables and a central commit | Current candidate gates inspect tokens/occurrences and owner commits; those tests establish their constructions. | E03/E04/E16: unique supported activation edges or local ownership, propagation diamonds, late partners/aliases and a conditional consumer. Compare logical firings rather than identical containers. | Exactly-once and legal consumption argument required; local finite probe feasible. Distributed completion is a separate follow-up. Open. |
| A6. Exact observation follows complete physical answer construction | E08/E10 export and E14 comparison expose costs but usually materialize ordinary answers first. | E14: exact joint-alpha/full-multiset checks on shared graphs before exporting only new answers. Include different sharing topology, repeated aliases, duplicate residual occurrences and mostly-distinct controls. | No solving/scheduling redesign needed. Retention and final exported bytes must be charged. Open. |
| A7. Search needs explicit complete branch snapshots and physical raw lineage enumeration | E00 lineage counts and current scheduling controls aid diagnosis; T011 already permits conditioned objects. | E03/E05/E07/E12: symbolic derivation/choice support and multiplicity, shared partial stores, demand-driven enumeration. Probe nested births, duplicate lineages, failure and infinite duplicate streams. | Preserve explicit source-choice correlation and required answer coverage; raw diagnostic counts do not by themselves require physical enumeration. Open. |
| A8. Fresh IDs, global rescans and normalized stores are necessary for completion | Current interfaces expose fresh numeric IDs, solved substitutions and complete-store checks; E09 has partial resumable services. | E01/E09/E16: safe reclamation, dependency-certified quiescence and local completion with unsolved obligations. Probe stale histories/subscriptions, pending clashes/cycles and divergent irrelevant-to-output active constraints. | Trustworthy answers still require all active obligations to be accounted for. No permission to ignore a loop simply because output is ground. Open. |

A8 has three separate subquestions and dispositions: A8a identity reclamation,
A8b dependency-certified completion, and A8c unsolved-store normalization. Evidence
for one does not discharge the others; each remains open.

These entries are contrasts, not selected architectures. A1 can inform A2/A3/A5,
but none of those depends on A1 succeeding. Likewise source layout, alias indexing,
choice support, scheduling and observation can vary independently; no one graph
representation supplies all answers. E17's independent application oracles remain
necessary across the alternatives.

## Correspondence independent of representation

An architectural candidate must explain the denotation of unfinished internal
state, not require every microstate to be a solved reference substitution.
Constructor relations may express an unsolved equation theory; event nodes may
express partially realized source effects. The argument must connect trusted
observations and progress to permitted source behavior, accounting for transient
states, failures and the scheduling assumptions under which obligations complete.

Preserve finite free constructors, pure/nonbinding head and guard tests,
multiset user resources, fresh-variable relationships, propagation behavior,
explicit disjunction correlation, selected outputs and full residual aliases.
Different coherent permitted schedules for a nonconfluent program are not
necessarily a mismatch. A matched-policy ablation may impose one schedule to
isolate cost, but that is local to the comparison. Changed language contracts need
separate experimental descriptions and owner adoption decisions.

For A1 specifically: repeated constructor descriptions must retain their meaning;
root equality requires corresponding children equal, incompatible constructors
refute, and positive constructor cycles refute only in the affected context.
Congruence closure alone is insufficient. Unknown source head arguments must not
acquire structure merely to make a rule match. A cycle in the union of incompatible
contexts is not proof of a projected finite-tree violation. Internal descriptors
need not be consumable source occurrences. Lowering, equality/alias maintenance,
matching indexes, occurrence ownership, support operations, cycle detection and
reification all count toward costs.

## Next selection and closure discipline

E18 first establishes an executable integrated relational store and adverse
correspondence cases, alongside authoritative source assessment. It must actually
permit constructor consistency and program rules to use one substrate; an
encoded unifier called through the current API would not answer A1. The first
semantic prototype may use explicit finite support sets, with their costs clearly
recorded, before a representation comparison. A competent fine-grained control is
required before partial-sharing or performance conclusions.

Each result updates the relevant A-entry as well as its E-family. Before closure,
state which assumptions the tested controls share and whether a feasible contrast
could materially change the conclusion. No entry is closed by this audit. The
regional repeated-cost proposal is queued because an untested architectural
contrast now has greater decision value, not because its evidence is invalid.
