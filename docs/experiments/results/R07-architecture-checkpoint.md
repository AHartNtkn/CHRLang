# Architecture decision evidence

Use competent explicit CHR execution as the general baseline, with selective
access, binding-aware activation where its scheduling contract applies, and
source-derived specialization. Add independently checked direct execution for
certified fragments. Keep Conditional as a separate bounded option for common
failure and lower peak heap; the evidence does not support making it the general
runtime. No workload weights or universal winner are inferred.

This recommendation includes current source lowering, storage and substantive
operation controls through T059. It is about supported architecture choices, not
proof that the prototypes are optimal or a production implementation is finished.

## Coherent alternatives

| Organization | Supported role | Necessary and chosen responsibilities | Contrary evidence / boundary |
|---|---|---|---|
| Explicit dedicated terms/equality plus selective CHR execution | General baseline | Finite-tree equality, branch-local bindings, source matching/effects, activation/index validity, source history and full observation; copying/COW is a storage choice | Index maintenance and poor keys can cost; unconstrained joins are not solved by one policy. Full branch ownership remains consequential. |
| Checked direct finite/recursive execution | Avoid general CHR machinery inside a proved fragment | Certificate preparation, direct equation/constructor or finite-domain service, exact observation and admission boundary | Does not authorize arbitrary eager contextual execution. Preparation must be amortized; unsupported sources retain ordinary execution. |
| Native generated recursive execution | Deployment/reuse option | Code generation, compilation/artifact ownership plus semantic services that remain | Measured native gains do not recover compilation versus the checked direct loop at registered reuse. |
| Conditional supported execution | Common pre-discrimination failure and lower peak-heap option | Support representation, scoped equality/effects, birth/history ownership, completion and answer enumeration | Loses all T059 successful and post-discrimination cells; ongoing history and publication costs constrain streams. No automatic mixed-engine routing is established. |
| Integrated constructor/equality/source substrate | Demonstrated capability, not current default | Integrated representation, validity and source-consumption correspondence | Current bounded comparison has contrary dedicated-control costs and higher ownership; further syntax coverage alone would not establish value. |

## Evidence that changes the choice

**Direct lowering earns more than code generation alone.** The
[recursive lifecycle comparison](R05-recursive-lifecycle.md) validates324 sessions
and62,316 observations. Direct beats Specialized in all eight nonzero-depth reuse
cells with separated ranges. Native improves Direct in three cells, but measured
compilation is not recovered within the registered reuse. The
[finite-domain comparison](R04-lifecycle-pilot.md) favors a dedicated trailed solver
in all20 cells of its independently certified closed relation fragment. These are
different facilities, not a universal solver or compiler ranking.

**Competent ordinary execution avoids substantial work without retained joins.**
[Access](R01-native-pilot.md), [nonfirst anchoring](R01-anchor-pilot.md) and
[immutable-subtree costs](R01-closed-subtree-lifecycle.md) establish useful mechanisms
and adverse regimes. The [current join screen](R01-current-join-screen.md) validates
32 processes: Active+Indexed takes7 candidate visits and10 cursor steps per
subsequent keyed request at both table sizes. Counts include producer service and
are not timing estimates. This family does not justify a maintained-join backend.

**Pure carrier work need not be executed repeatedly.**
[Carrier costs](R05-carrier-cost.md) favor certified lowering in14 ground cells.
The [known-prefix comparison](R05-carrier-prefix-cost.md) validates192 processes:
ground controls favor Carrier; long unknown timings overlap Conditional; short
unknown cases favor Conditional. Lowering preserves actual tails, aliases, IDs and
ordinary arbitration under its singleton/source certificate. It does not establish
arbitrary contextual contraction or mandatory source restrictions.

**Substantive shared equality does not imply a general shared-runtime win.**
[T059](R05-equation-cost.md) validates576 processes and72 cells. Conditional wins
all six common clashes before discrimination; ordinary and COW Specialized win the
other18 cases, including every successful query. At depth64 cold, before-clash
medians are0.231ms Conditional and0.701ms Specialized; before-success instead favors
Specialized,0.992ms versus1.150ms. These figures belong to this freeze only.
Conditional performs one common substantive equation before discrimination, but
shared equation work does not offset the remaining complete-path costs. Its failure API exposes
query exhaustion rather than one event per failed explicit leaf; the comparison
preserves and reports that distinction.

**Ownership is a separate tradeoff.** [Arena COW](R03-arena-ownership.md) has eight
separated read-choice benefits, one no-choice regression and23 overlaps including
all insertion cells. T059 COW/ordinary timing ranges overlap in all24 pairs. At
T059 depth64 cold before-clash, Conditional requested peak is90,340 bytes versus
738,905 ordinary Explicit, but lower peak is not generally lower traffic: after-clash
Conditional allocates17,398,205 requested bytes versus1,235,390. Requested heap is
not RSS. Choose storage from mutation/lifetime evidence, not a universal sharing rule.

**Capability is not default preference.** [Integrated congruence](R02-congruence-witness-pilot.md)
permits real interleaving of equality and consuming source effects, with contrary
cost evidence. [Stream lifetime](R06-streaming-lifetime.md),
[restricted publication](R06-restricted-publication.md) and
[finite-sibling publication](R06-publication-flow-gate.md) expose retention and
progress costs. Use explicit execution as the supported low-sharing stream control.
Current [worker comparisons](R01-closed-subtree-lifecycle.md) leave concurrency
benefits unresolved; factoring and concurrency are distinct decisions.

## Why further implementation is not presently selected

The [all-question sufficiency assessment](R07-sufficiency-current.md) accounts for
every Q1–Q12 direction, including the completed substantive comparison. Plausible
extensions are retained with their reopening conditions:

- Maintained weakly keyed/many-to-many joins need a concrete surviving update cost.
  Current keyed requests do not supply it; porting the historical retained-prefix
  mechanism would add validity and storage before establishing a benefit.
- Shared-representation equation or clash reuse is credible. [E12](E12-equations.md)
  and [selective failure checking](E12-failure-native.md) establish source-boundary
  precedent and checking/interface costs. A cache could narrow the common-failure
  exception, but needs valid identities, dependencies and deferred effect ownership.
  It would mainly strengthen the general explicit recommendation; no supplied
  workload distribution makes eliminating that exception necessary now.
- Broader contextual lowering, multiple carriers, integrated search or direct
  choice graphs need new correspondence and a decision-changing source regime.
  Demonstrated capability or prototype availability alone is not enough.
- Warm worker pools, larger compilation reuse and additional application suites
  need deployment or computational requirements. Another favorable cell would
  not supply missing workload weights or establish a general ranking.
- General reclamation, tables, replay and new publication policies need a surviving
  lifetime/recomputation case. Their semantic and ownership obligations are real;
  the supported low-sharing control does not currently require those expansions.

These are bounded cost/value judgments, not impossibility claims. Reopen them when
an actual architecture or workload decision changes their premises. Do not repeat
measurements merely to separate overlapping ranges or erase contrary evidence.

## Language and adoption decisions remaining with the owner

1. Whether to adopt the explicit baseline with optional checked finite/recursive
   facilities. Each certificate's source and observation boundary is explicit;
   broad eager contextual execution has not been justified.
2. Whether common-failure or peak-heap requirements justify supporting a separate
   Conditional mode alongside its support/history machinery. No transparent
   regional routing or interoperability policy is established by these experiments.
3. Whether the intended mutation/lifetime pattern warrants arena COW. The evidence
   supports a selectable representation, not an unmeasured adaptive policy.
4. Whether native artifacts or a different deployment/reuse regime are required.
   Such a requirement can reopen compilation or worker measurements; it is not
   inferred from examples.

The experimental source contracts continue to preserve nonbinding matching,
finite-tree equality, aliases, raw successful multiplicity, branch-wide failure,
and stated source/progress obligations. API compression, answer quotienting or
stronger language restrictions would be separate owner-adopted changes, not
unannounced performance controls. The objective here is evidence for those choices;
choosing a production workload distribution or implementing the chosen product is
not silently made a prerequisite for completing that evidence.

## Measurement and reproducibility limits

Registrations precede comparative runs; original raw results, hashes, commands,
failures and validation receipts remain authoritative. Current native lifecycle
runs separate counter-free ordinary-allocator timing from work/allocation builds,
charge preparation/query/service/observation/disposal, and validate full answers
outside intervals. Compilation was measured in the recursive artifact experiment;
it is not credibly isolated in every later prototype comparison. No claim of
complete architectural lifecycle superiority bridges that gap.

Earlier Python/instrumented and component probes retain only their stated scope.
No cross-freeze numerical ranking, aggregate score, confidence interval from a
handful of repetitions, RSS claim from requested allocation, or universal
application distribution is inferred. Reference algorithms remain independent.
The [closure audit](R07-closure-audit.md) records objective-level verification.
