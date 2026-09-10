# Direct solving: source obligations before implementation

This is a semantic experiment, not a cost comparison or a solver implementation.
T073 remains active. Test which transformations a source-derived finite solver
must distinguish before spending on its implementation.

Hypotheses:

1. In the existing closed choice/check source, replacing the coin alternatives
   with their sole successful binding at the original rule location preserves
   full successful answers. This is a test transformation with a known premise,
   not an implemented inference algorithm.
2. A linked consumer that rescues a failing check invalidates that premise.
3. Moving the successful binding to an earlier source location can enable a
   competing consumer/writer before the original choice consumes its resource.
4. Duplicate successful alternatives remain duplicate answers; a set of satisfying
   assignments is insufficient when source derivations have multiplicity.

Controls: independent owned-syntax scalar semantics and existing compiled Global
Scan, with complete outputs, aliases and residual multiplicities compared through
exact observation equivalence. No reference-interpreter changes. Closed-source
matrix: both arrival orders, depths 0..6, work 0/2, resources off/on, final failure
off/on and both query tags: 224 configurations. Additional named tests challenge
linked consumers, early binding and duplicate alternatives, including aliased
requests. Run default and counter-free builds. These deterministic tests each run
once per build; no timing inference. Scalar bound 2,000,000 service steps and
compiled bound 200,000 calls per execution; a cutoff is a failure. Bound the whole
test command to 180 seconds. Inspect failures before extending bounds.

Selection: direct solving could eliminate exponential alternatives still present
in the qualified compiler controls. These small source probes can prevent an
unsound implementation cheaply. Native graph/connected feasibility is the
strongest distinct alternative and remains required; broader reuse and finer
effect precision are also unresolved. At this gate decide whether a concrete
source-derived solver is feasible enough to implement next, and explain its
advantage over those alternatives. Passing examples do not prove general
eligibility or justify rejecting an architecture.
