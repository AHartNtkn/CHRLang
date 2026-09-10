# Native identity must preserve occurrence history and effect order

The new source checks distinguish occurrence replacement from value binding, and final equality from legal effect order. These distinctions are absent from the current native ground compiler. They define what its next representation must preserve; they do not qualify a native implementation.

## What the source requires

| Distinction | Complete source result | Consequence for a native design |
|---|---|---|
| Replace `p(a)` with a new `p(a)` after propagation | Each replacement permits another `seen(a)`; up to four receipts are checked | A history keyed only by the current value would suppress a legal application |
| Bind an existing `p(X)` to `a` after propagation | The existing receipt becomes `seen(a)`; propagation does not fire again | A changed value must not automatically create a new occurrence history |
| Replace `p(X)` with `p(Y)` where Y is fresh | The old observed unknown and the replacement's unknown remain distinct | Reusing a value slot must not capture an earlier source variable |
| Propagate over two kept heads | Distinct ordered occurrence tuples matter, even when all values equal `a` | One occurrence cannot fill both positions; equal values do not collapse receipt multiplicity |
| Compare initially aliased and distinct unknowns | Final aliases agree, but different consumers win | Final equality is insufficient to reconstruct legal effect order |
| Bind two independent unknowns through disjoint source heads | Both source orders produce the same complete answer | Some effects do commute; the adverse case does not establish universal serialization |

These are requirements on observable distinctions, not prescriptions for a particular data structure. Explicit occurrence IDs and rule/tuple histories are one implementation. A compressed or aggregated representation remains valid if it preserves the same applications, aliases and multiplicities.

## Why equal final bindings do not settle ownership

The discriminating source first tries `p(X), q(X) <=> joined(X)`, then `p(X), token <=> taken(X)`, then a rule that equates the arguments of `bind(X,Y)`. Its query contains p, q, bind and token.

When p and q initially share an unknown, the first rule consumes both and leaves token. When their unknowns are distinct, that match is not yet entailed: the second rule consumes p and token. The binder later makes the remaining values equal. Both executions end with identical selected-variable aliasing, but one contains `joined` and the other contains `taken` plus q.

Moving the binder to the first source-rule position changes the initially distinct case to `joined`. That is a valid result for the reordered source, not for the original source. A local protocol that permits the lower-priority binding effect to overtake an already enabled consumer can therefore change the program.

Per-occurrence consumption claims alone do not address this distinction: the binder consumes a different head but changes equality used by other matches. A protocol needs a sound way to preserve the specified selection/effect order or prove the relevant actions commute. This does not prove that a global lock, a global barrier or a particular version table is necessary.

## History and equality are different responsibilities

The replacement and binding tests impose opposite requirements on history maintenance. Replacing an occurrence with an equal-valued one enables propagation again; changing an existing occurrence's value does not. A single equivalence relation based on current values cannot represent both distinctions.

For two kept heads and n identical occurrences, the checked source produces n(n−1) receipts for n from0 to3. With distinct values a and b, both `seen(a,b)` and `seen(b,a)` remain. A symmetry-aware compiler could aggregate equivalent applications with multiplicity, but simply treating the tuple as an unordered set would lose this source observation.

Fresh replacement adds another distinction: an earlier output alias must continue to name its original unknown. The newly introduced variable is shared between the replacement p and its new receipt, without becoming the earlier unknown. Subsequent equality could relate them, but allocation or slot reuse must not silently do so.

## Validation scope

Six tests cover61 source configurations, varying query order, variable numbering, replacement count, tuple multiplicity, initial aliasing and source-rule priority. Every case checks the independently written complete residual, joint output aliases, one raw completion and exhaustion within20000 reference steps. All15 tests in the containing package pass. Scoped strict Clippy and formatting pass.

The [tests](../../../research/chr-cases/tests/native_identity_obligations.rs) use the unchanged reference engine and owned language syntax. This is source-obligation evidence. No native identity representation, local-commit protocol, cancellation mechanism or performance comparison was implemented by this package. In particular, these passing reference tests must not be reported as native source correspondence.

[Registration](../registrations/S03-native-identity-obligations.md), [validation and source hashes](s03-native-identity-obligations/validation.json), [package output](s03-native-identity-obligations/tests.log) and [Clippy output](s03-native-identity-obligations/clippy.log) retain the evidence.

## Four-package breadth review

The four packages since native execution was selected are retained leaf service, structured observation, ground consuming compilation and these identity/effect obligations. Native service and bounded ground source execution are now demonstrated. A broader native candidate still needs source-visible identities, equality, propagation and effect ordering; the final package identifies that work rather than completing it.

**Select T072's complete native timing comparison for general integrated matching next.** Its [ownership gate](S02-multihead-ownership.md) already compares seven complete paths with independent observations, changed queries, prepared reuse and exact allocation/restoration evidence. Sparse/nested cases favor partial joins in traffic, while dense/cold cases favor conventional scanning. The missing timing comparison can determine whether retained matching repays its setup and maintenance. Revalidate the current sources and extend the ordinary-allocator runner before prospectively registering timings.

| Ready investigation | Decision it can change | Priority at this boundary |
|---|---|---|
| General integrated matching lifecycle timing | Whether reduced matching/traffic justifies retained environments, preparation and repair in complete queries | Selected: existing source and allocation gates lower the cost of a credible total-efficiency comparison |
| Native identity/equality/history implementation | Whether native choice execution can support the broader language with acceptable responsibilities | Required; building and validating that representation is a larger next package, with costs still unknown |
| Finite-solver solution ownership, admission and learning | Whether current selective gains survive broader source and lifecycle demands | Required; current gains and unselective/cold costs remain bounded evidence |
| Broader reuse and effect precision | Whether validity, ownership or inference removes additional work economically | Required; no direction is rejected by this priority change |

The timing study requires counter-free ordinary-allocation builds and separate work/allocation diagnostics. Keep conventional Scan, Indexed and applicable specialized controls. Charge preparation, changing-query setup, execution, complete observation and disposal. Preserve cold/dense adverse cases and investigate consequential uncertainty; lower allocation is not a speed result. Existing allocation measurements do not cover sustained consumers or general search, which remain separate obligations.

T080 becomes pending, not complete. Resume it with an explicit identity-bearing source representation, using the cases above alongside the compound source gate, and compare serial versus local ownership on actual source effects. The broader native path has neither won nor lost a complete architecture comparison. Coherent architectures, language tradeoffs, held-out challenges and every consequential unanswered direction remain required under the [governing sequence](../sequence.md).
