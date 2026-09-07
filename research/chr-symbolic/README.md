# Bounded symbolic execution experiment

This directory currently contains a finite-slot unification circuit, the first correctness gate for E11. It is not yet a CHR evaluator. The constructor arena is finite, immutable and topologically ordered; variable slots can acquire substitutions. Equation operands are symbolic finite indices. Explicit microstep bounds return cutoff separately from failure and completion.

Create an isolated Python environment and install `requirements.txt`; run `python -m unittest discover -s research/chr-symbolic` from the repository root. The current environment is `/tmp/chr-symbolic-venv`. Tests compare all 121 pairs in an 11-node arena with an independent nested-tree Robinson implementation. A zero-step equation validates cutoff. No comparative performance claim follows from these tests.

The circuit uses explicit SSA equations for substitutions, stack cells, length and failure. Occurs checking computes bounded reachability in the pre-binding acyclic graph. A path has fewer than the arena's node count edges, so that many iterations suffice. Stack capacity covers the maximum growth across the registered number of microsteps. Constructor tags include arity. Unbound variables decode as variable handles, not arbitrary solver-chosen ground terms.

Next: sequential equations and mutable arena allocation, independent decoded transition checks, pending goals, occurrence identities, nonbinding matching, committed rule selection, history, explicit choices and quiescence. Freeze a comparative matrix only after these semantics pass. See `docs/experiments/registrations/E11.md`.
