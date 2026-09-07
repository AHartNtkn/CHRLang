# E11 registry inventory and application-bound diagnosis

The symbolic compiler completes 23 of 64 E00 cases at the small registered bounds, with full expected answers and no cutoff. The other 41 explicitly report bounded incompleteness. There are no errors or semantic mismatches in this inventory. Application coverage remains incomplete.

## Evidence and reproduction

[Small registry inventory](E11-registry-small-v1.jsonl) uses T8,N6,O3,P3,U4 and a 60-second fresh-process limit. The complete cases include resource identity, inherited/conditional propagation history, committed selection, nonbinding matching, correlation, raw duplicates, alpha residuals, equality/guards, and a suspended lambda normality constraint. The suspended constraint is not evidence for completing lambda evaluation.

Use `cargo run --quiet -p chr-symbolic-fixtures --example export_cases` to generate the manifest and `run_conformance.py MANIFEST OUTPUT --all` in the isolated Python environment. The core encoding for this inventory is `df0f939`; `960845c` records the expanded harness, bound derivation and subsequent literal-selector shortcut. Timings are exploratory diagnostics.

[Derived bounds](E11-derived-bounds.jsonl) come from `derive_bounds.py`, which independently executes all 64 cases and checks expected answers, raw multiplicity and exhaustion. T is maximum branch trace length; O is monotone introduction count; P is maximum pending length; N counts initial constructors/variables plus constructors and fresh variables instantiated by fired rule bodies. Both OR arms count in this encoding. Two manually calculated tests check arm freshness/constructors and guard-only fresh variables. U8 is an exploratory service bound, not an adequacy proof.

Selected derived bounds:

| Case | T | N | O | P |
|---|---:|---:|---:|---:|
| add-forward | 19 | 27 | 3 | 3 |
| add-decompose | 25 | 31 | 4 | 3 |
| type-synthesis-prefix | 7 | 24 | 1 | 3 |
| SK identity | 69 | 446 | 7 | 4 |
| SK ignored hole | 234 | 1,302 | 33 | 4 |

These counts describe the present eager body-allocation representation, not an inherent lower bound on CHR evaluation. The prefix case remains a prefix even if a bounded answer matches its expected first answer.

## Application probes and profile

[First application runs](E11-app-derived-v1.jsonl) and [literal-selector follow-up](E11-app-derived-v2.jsonl) each hit the 60-second limit on add-forward, add-decompose and type-synthesis-prefix. The first harness did not capture phase. Flushed phase markers in the follow-up establish that all three stop during construction. The literal selector returns the same vector cell directly when its index is already known; all 25 Python tests pass after this shortcut. It is insufficient to resolve these application runs.

A [four-transition addition-prefix diagnostic](E11-add-prefix-profile.json) at N27,O3,P3,U8 completes with no resource cutoff and an expected transition cutoff. cProfile records 40,638,414 calls and 13.403 seconds; ten `View` constructions account for 12.490 seconds. This prefix has not executed a source equation. Formula construction takes 5.723 profiled seconds and observation preparation 7.640 seconds. Profile timing is diagnostic and overlaps another experimental process; no architecture speed ratio follows.

Reproduce profiling at `960845c` using `python -m cProfile -o PROFILE research/chr-symbolic/probe.py MANIFEST app-add-forward --transitions 4 --nodes 27 --occurrences 3 --pending 3 --service 8`.

## Resulting next investigations

Avoid repeated symbolic projection when all heap fields are already concrete. Decode model heap fields directly for independent witness replay rather than symbolically constructing every snapshot's datatype values before solving. Both retain variable identities and full residual semantics and can be checked against the existing replay service. Measure construction/projection changes before interpreting application timeouts as evidence about the broader bounded-compilation direction.

Eager allocation of constructors in both arms and the dense heap representation also require investigation, particularly for the SK bounds. Feasible alternatives include delayed term construction from templates, constructor reuse and more compact heap projections. These questions remain active; an unfinished application probe is not closure. Equal-bound performance controls, increasing bounds, prefix reuse and full intended application coverage remain required for E11.
