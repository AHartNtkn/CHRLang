# Source analysis avoids update work without requiring native compilation

Source-derived update plans preserve the tested answers and avoid substantial unnecessary index and dependency work. A prepared data plan achieves the same work reduction as generated update code. Native compilation must therefore be compared against that stronger control; these counts do not establish a speedup.

The investigation also confirms a scheduling constraint: an eligibility-irrelevant binding can change consuming competition by waking an occurrence. The existing Active policy cannot omit that wake on eligibility grounds alone. T070 remains active for full cost comparisons.

## The comparison

The [source analysis](../../../research/chr-compiled/src/generate.rs) identifies a conservative set of usable index columns. A variable appearing only in the selected head has no value in the binding frame before candidate lookup. An argument containing that variable therefore cannot supply a complete ground key. Ground source arguments and arguments whose variables occur in other heads remain eligible; requirements are unioned across every use of the predicate.

This is an overapproximation across source prefixes and anchors, not a minimal index set. It depends on the present lookup plans: guards do not establish bindings before lookup. Guard pushdown, constructor-prefix indexes or other access plans require a corresponding maintenance analysis.

There are now three repair controls within the existing executor:

| Repair organization | Preparation and runtime responsibilities |
|---|---|
| Generic repair | Traverse every head argument and maintain every argument's ground key |
| Prepared column plan | Analyze the source once; retain a predicate-to-column map; interpret that map during repair |
| Generated repair | Emit per-predicate watch/key calls; link a predicate-to-function map during preparation |

Both projected paths use the [same repair primitives](../../../research/chr-compiled/src/native_updates.rs). They retain all argument watches under Active scheduling. Under Global indexed execution, they watch the columns needed for index repair; source selection resumes after equality work. The prepared control uses ordinary generic matching, while the native control uses generated continuations. The payload gate also compares the same native continuation with generic repair, separating update organization from matching organization.

The generic repair implementation remains available as a control. The projected paths also use different scratch storage and omit empty dependency/key records. A later allocation comparison must account for those implementation differences; this gate does not attribute a hypothetical timing benefit solely to column removal.

## Observed work and source effects

The payload source publishes an opaque value through a two-head join, then binds its variables individually. Those variables remain present in the full residual answer but never supply lookup keys. A late-key variant separately checks bindings that really must enable matching. There are 32 width/key/policy/access configurations, with independent scalar observations and exact source-trace comparisons.

| Indexed configuration | Generic repair dependency visits | Both projected paths | Interpretation |
|---|---:|---:|---|
| Global, 16 payload variables, initially known key | 475 | 2 | Avoid repeated traversal of unrelated payload bindings |
| Global, 64 payload variables, initially known key | 6,499 | 2 | The avoided work grows with payload size and repeated binding |
| Global, 64 payload variables, late key | 6,632 | 4 | Retain the necessary key dependency while omitting payload dependencies |
| Active, 64 payload variables, initially known key | 6,499 | 6,499 | Preserve the established wake ordering |
| Active, 64 payload variables, late key | 6,632 | 6,632 | Key projection alone does not justify selective Active wake-ups |

The final payload state retains two index entries in either projected path, versus three under generic repair. At width 64 with an initially known key, Global index insertions fall from 131 to 2; Active insertions fall from 131 to 66. The three-edge chain retains seven entries instead of ten. All are diagnostic work/state counts, not requested bytes, RSS or elapsed-time comparisons. [Raw work checks](s01-generated-update-gate/final-work.json)

The prepared data plan and native path also pass the existing 192-configuration finite source corpus. Prepared plans join the explicit-search gate, with independent branch replay across ten sources, both policies and both access modes. Both projected paths join the live-demand source checker, including consumption, propagation, retirement and late binding. The separate native artifact passes 116 changed-query, access and scheduling checks.

## Why the Active wake matters

Consider these competing rules after `p(X)` has already been tried without a `q`:

```text
p(X), q <=> left
r, q    <=> right
seed(X) <=> X=a, r, q
```

Binding X does not change the first head's eligibility. Nevertheless, the existing Active implementation wakes p before the newly inserted r. Once the pending body is posted, p consumes q and produces `left`. Omitting p's watch lets r consume q and produces `right`.

The deliberate omitted-wake variant fails the independently compiled artifact on precisely this Active trace: it chooses rule 1 rather than rule 0 after the seed. This is a concrete limit on preserving that policy, not a rejection of all selective wake-up designs. A confluence or noncompetition argument, or a different explicit scheduling contract, could support a different choice. [Counterexample receipt](s01-generated-update-gate/omit-active-wake.json)

The second fault omits Global key watches. It misses enabled applications in the delayed-edge source after equality changes a key. Thus the gate distinguishes unnecessary payload watches from required index repair. [Missed-key receipt](s01-generated-update-gate/omit-global-key-watch.json)

## Validation and remaining costs

The [validation manifest](s01-generated-update-gate/validation.json) records final default, counter-free release, source/artifact, feature and Clippy checks. Source hashes identify the tested implementation and independent oracles. The earlier continuation gate resolves its own sources at commit `7b8f469`.

The new immutable update map is included in fork ownership diagnostics. The classic and copy-on-write diagnostic runners account for its clone boundary and restore their allocation baselines across two checked query lifetimes each. Those runner cases have no generated map installed; they validate the common ownership accounting, not native prepared-map allocation cost. Native and prepared-map branch/source checks exercise their installed maps separately. Initial ownership-accounting and generated-format diagnostics, and their final checks, are retained in the receipts.

The next cost comparison must include the prepared data plan, compiled matching with generic repair, compiled matching with generated repair, and competent indexed/retained source controls. Carry source-eligible single-head specialization into the controls. Charge source analysis, map construction, generation, compilation, query setup, execution, first/full observation and disposal across changed queries. Shared runtime compilation must be distinguished from each user-program artifact; the current artifact gate's reusable build cache supplies no isolated compilation measurement.

Current native preparation still constructs generic rule metadata. Its unused templates and the generated lookup plan's treatment of endpoint information are possible cost limitations to inspect during sizing, not inherent costs to assume for compilation. Tiny/cold and fully relevant-argument sources must accompany the private-payload favorable case. No comparative timing matrix for this candidate has run, and the broader architecture choice remains open.
