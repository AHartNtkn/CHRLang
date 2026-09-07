# R04: source execution agrees with the finite relation

The finite relation agrees with source execution on the registered consistency cases, including complete residual multisets and hidden choices. This supports using it to compare direct solving with rule execution. It does not establish that solving is faster or that the lowering extends to general CHR.

## Evidence and scope

The [prospective gate](../registrations/R04-source-correspondence-gate.md) ran 338 queries under each of six rule orders. All complete observations, raw successful completion counts and exhaustion flags agreed. Every supplied query without choosers produced zero source splits. No case reached its bound or required a semantic repair during this run.

The queries include four contrasting four-variable networks with choices, all 324 supplied assignments across those networks, and ten boundary cases. The boundary cases distinguish repeated choosers, repeated residual occurrences, aliases, unobserved choices, conflicting givens, self-edges and ground/empty queries. Rule permutations check the correspondence's order claim; they are separate test executions, not source search over schedules.

The [assignment oracle](../../../research/chr-direct-solver/oracle.py) uses Cartesian enumeration and direct tuple checks. The [Rust bridge](../../../research/chr-direct-solver/src/main.rs) constructs source syntax and runs the independent copying reference, then decodes domain atoms and residual occurrences. Neither path imports the other's evaluation algorithm. The [runner](../../../research/chr-direct-solver/source_gate.py) supplies the same query identities to both and compares entire normalized observations. Its normalization sorts residuals while retaining multiplicity. Answer deliveries are normalized to a set: this gate does not independently establish that the source never delivers a duplicate recognized answer.

The bridge's empty query yielded one empty completion; a true ground forbidden tuple yielded none. These analytic checks precede the registered gate. Twelve focused oracle tests pass, including source output eligibility. Rust formatting and Clippy for this package pass. These checks validate this gate; they do not replace the reference interpreter's broader semantic tests.

## Reproduction

The [machine receipt](../../../research/chr-direct-solver/results/source-gate.json) contains the query count, rule orders, completion status, empty failure list, exact input and binary hashes, the executed command, and source hashes for the oracle, bridge, runner, reference and syntax. Its revision identifies the committed base; hashes identify the task-owned source used before commit.

```sh
cargo build -p chr-direct-solver
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-direct-solver -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-direct-solver/source_gate.py \
  --receipt /tmp/r04-source-gate.json
```

A rebuilt binary's hash can depend on toolchain/build details. Source hashes and exact semantic checks determine reproduction; no runtime ranking is inferred from this debug build.

Independent review found no material correspondence defect and independently verified the recorded sources, binary, reconstructed input and query registry. It confirmed the answer-set scope above.

## Architectural implication and next choice

A direct solver for this fragment may omit rule dispatch, occurrence matching and propagation history from its search, provided it reconstructs the original residual multiset. The [correspondence argument](R04-finite-consistency-entry.md) explains why this is valid. The tests corroborate that argument against the implemented source semantics across the registered cases.

The open question is whether this elimination pays after domain propagation, enumeration, compilation and reconstruction costs. A useful next comparison must give ordinary execution competent access/activation and give both competitors equivalent static information. Comparing direct solving only to this deliberately simple reference would confound eliminating execution with elementary lookup and preparation improvements.

A small direct solver versus a competent execution control is now eligible for a decision brief; no cost run is selected by this receipt alone. Sparse supplied queries and broad, weakly selective constraints must accompany coupled search. R02 class/index integration remains a competing use of implementation effort, and R01 supplies evidence about compiled execution without being a prerequisite for R04's semantics.
