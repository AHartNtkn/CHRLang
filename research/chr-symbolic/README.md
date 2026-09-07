# Bounded symbolic execution experiment

This directory currently contains a finite-slot unification circuit, the first correctness gate for E11. It is not yet a CHR evaluator. The constructor arena is finite, immutable and topologically ordered; variable slots can acquire substitutions. Equation operands are symbolic finite indices. Explicit microstep bounds return cutoff separately from failure and completion.

Create an isolated Python environment and install `requirements.txt`; run `python -m unittest discover -s research/chr-symbolic` from the repository root. The current environment is `/tmp/chr-symbolic-venv`. Tests compare all 121 pairs in an 11-node arena with an independent nested-tree Robinson implementation. A zero-step equation validates cutoff. No comparative performance claim follows from these tests.

The circuit uses explicit SSA equations for substitutions, stack cells, length and failure. Occurs checking computes bounded reachability in the pre-binding acyclic graph. A path has fewer than the arena's node count edges, so that many iterations suffice. Stack capacity covers the maximum growth across the registered number of microsteps. Constructor tags include arity. Unbound variables decode as variable handles, not arbitrary solver-chosen ground terms.

Next: sequential equations and mutable arena allocation, independent decoded transition checks, pending goals, occurrence identities, nonbinding matching, committed rule selection, history, explicit choices and quiescence. Freeze a comparative matrix only after these semantics pass. See `docs/experiments/registrations/E11.md`.

The equality circuit now composes a fixed number of sequential equations, carrying substitutions and preventing execution after failure or cutoff. Additional tests cover late occurs failure and cutoff propagation.

`machine.py` supplies a separate tree-based direct transition service with committed rule/tuple selection, explicit OR, occurrence identity and propagation history. It retains raw answers and traces for validation; it is not yet the registered performance control. `verify_witness` checks every supplied state, action and endpoint, rejecting altered history, wrong choices and unfinished endpoints.

Cross-check all 64 E00 cases using:

```sh
cargo run --quiet -p chr-symbolic-fixtures --example export_cases | /tmp/chr-symbolic-venv/bin/python research/chr-symbolic/check_cases.py
```

The exporter calls the Rust reference through its public interface and includes the independent registered expected answers. The Python checker compares full residual alpha equivalence, raw completion counts and exhaustion. Its observer does not import the reference canonicalizer. Current validation covers 2,771 direct transitions across the registry. The symbolic whole-machine circuit is still to be implemented; these checks do not transfer correctness automatically to it.

`heap.py` now owns the symbolic equality implementation. It adds guarded finite-slot allocation, explicit variable identities, substitutions and constructor children. The fixed-arena `Circuit` is a test harness over this service. Guarded allocations update the actual branch's counters, so fresh variables retain distinct identities even when alternatives allocate different numbers of nodes. Inactive equations neither bind nor spend their service bound. Active allocation overflow propagates cutoff; it cannot produce a successful endpoint.

The heap is a component of the pending whole-machine implementation. It has no rule scheduler or answer enumeration of its own. Constructor references must point to allocated slots; substitution edges may point forward, with occurs checks maintaining finite-tree validity. Building fresh constructors after earlier bindings and branch-local occurs failure are covered by executable checks.

`matching.py` performs nonbinding matching and pure guards over deterministic finite-tree views. `symbolic.py` now compiles the initial whole-machine transition circuit, including occurrence consumption, history, choices and quiescence. Each emitted model undergoes independent per-state replay. Four initial machine cases pass; broader application conformance and the comparative matrix remain pending. Constructor projections use explicit Hole IDs for free variables, preserving nonground answers.
