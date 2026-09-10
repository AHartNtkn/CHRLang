# Validate recorded relevant reads before constructing another key

T072 tests whether cached equality deductions can be recognized without rebuilding their complete reachable-state key on a hit. The prior relevant allocation attribution identifies a consequential cost. The archived resident restoration pilot has already priced its tested checkpoint regimes; its repeat is less valuable than this distinct recognition mechanism. Larger retained-state restoration, switching policies and sustained consumers remain required.

## Candidate and soundness obligation

Preserve the current relevant-key cache and local deduction replay as controls. Add an experimental lookup policy that ranges over cached entries with the same ordered root inputs. For each candidate, check every recorded value's representative and complete constructor descriptions against the current context. Accept only an exact match. On no match, construct the original complete key and execute/record normally. Keep the 4096-entry bound and caller-local maps/resources.

The saved read set is transitively closed over constructor children reachable from both inputs. If all recorded representatives and complete descriptions still match, no newly reachable child can appear without changing a checked description. Descendant changes, added cycles and incompatible root descriptions must reject reuse. Stable arena identities are scoped to the shared arena; this is not cross-program identity or continuation reuse.

Ordered maps compare borrowed descriptions. Persistent map lookup currently returns owned values, so that variant may still clone descriptors while validating; measure this cost rather than call it allocation-free. Lookup scans same-input variants and may lose on many near misses. No second executor or production baseline is introduced.

## Gates before costs

Extend the existing relevant-deduction suite across rebuilt-key and validated-read policies with ordered and persistent equality maps. Preserve independently expected bindings, duplicate resource claims, incompatible descendants and occurs failures. All 48 complete-source configurations must agree with the independent scalar evaluator and existing Scan/Indexed controls; include actual changed-context reuse. A separate diagnostic test must demonstrate a repeated merge with a cache hit and no key-construction scope, while leaving complete output correct.

Run counter-free and `deduction-work,deduction-profile` tests, with diagnostic counters never used for primary timings. Preserve a failing test before implementation. Mutation-check omission of descendant validation against the occurs-failure counterexample in a temporary isolated source copy. Compile and run the unchanged reference only where needed by existing controls; do not modify it.

Bound each test process to 120 seconds wall/CPU and 1 GiB where compatible; compilation has a separate 300-second wall bound. No comparative timing is registered here. Next qualify ownership/allocation and lookup-versus-key construction on repeated useful calls, unique requests and many same-input near misses before registering total timing. A semantic pass cannot select this policy or close the architecture question.
