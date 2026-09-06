# Comparative dossier review

The dossier is ready for review as a proposed semantic comparison. It supports further analysis of specific correctness and eligibility obligations, but does not yet justify prototype selection or a language restriction. Broader literature coverage remains an explicit M1 gap.

## Review scope and corrections

On 2026-09-06, a separate agent reviewed the reference and candidate descriptions against the charter and intended programs. It had contributed the candidate analysis and trace, so this is a cross-check rather than wholly independent authorship or external source certification. The PM checked the hand-derived trace and incorporated all three required precision corrections:

- Fresh occurrence identities are allocated relative to the entire derivation, preventing collision with tokens referring to consumed occurrences.
- Unification composes substitutions and maintains a solved acyclic interpretation.
- Liveness is relative to the declared committed schedule and finite-work assumptions; it does not promise exploration of other CHR schedules.

The reference also makes nonempty heads, unique rule names, freshened variable allocation and nonbinding head matching explicit. Worked cases cover conditional consumption, histories acquired before and after a choice, finite-tree occurs failure, relevant context, fresh variables and nonground answers.

The review judged the candidate comparisons fair: explicit relational alternatives do not themselves require overlapping heads; compiler specialization need not force users to write inverse relations; and local solver restrictions must not be silently imposed on the entire language.

## What this checkpoint establishes

**C01, semantic fidelity:** a proposed reference adaptation and specific correspondence obligations exist. There is no full representation theorem or implemented oracle. Additional built-ins remain parametric; their concrete catalogue requires a later definition.

**C02–C04, space, shared work and net benefit:** candidate costs and adverse cases are identified, including conditional data-structure overhead and lost pruning through abstraction. No project performance conclusion is supported.

**C05, answers:** the draft supplies a conservative observation and a sufficient structural equivalence criterion. Display projection and continuation APIs remain distinct open issues. The criterion does not decide arbitrary semantic equivalence.

**C06, language costs:** the comparison identifies properties that could be inferred, locally certified, or globally required, and relates their consequences to arithmetic and synthesis. No restriction is adopted.

## Coverage and next work

Two bounded expansion rounds investigated author/forward followups and primary leads across functional logic, local-net resource analysis, conditional structures and tabled constraints. The source record reports stabilization of the sampled mechanisms while retaining access and broader citation-coverage gaps. This is not a global saturation claim.

The next research work is specific: define and analyze conditional substitutions and occurrence updates; derive sufficient dependency/eligibility conditions for the relation and local-net candidates; specify a solver interface before claiming tabled or learned reuse; and resolve the remaining foundational/forward-source gaps where they affect those claims. Each outcome should be a proof argument, counterexample, or precisely scoped unresolved obligation. More favorable examples alone would not settle it.

M2 can select bounded prototypes only after those candidate contracts and experiment questions are reviewable. No owner taste decision is required to conduct this analysis. Query-operation and answer-interface decisions must be presented when an implementation contract depends on them.

## Artifacts and validation

- [Reference and candidate contracts](T008-reference-and-candidates.md)
- [Worked obligations](T008-worked-obligations.md)
- [Coverage and source ledger](T009-coverage.md)
- [Intended programs](intended-programs.md)

Validation consists of substantive review, manual trace checks, local-link validation, task-board schema checks and whitespace checks. These validate the dossier's consistency, not its unproved mathematical or performance claims.
