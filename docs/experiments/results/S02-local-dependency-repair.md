# Constructor waits no longer wake on irrelevant aliases

The local integrated executor now distinguishes missing constructor information from unresolved equality. At width 64, the broad constructor witness uses **128 request inspections instead of 2,207**, with the same complete outputs and token consumption. A separate equality witness still exposes quadratic reinspection, so this is a bounded repair rather than a general dependency solution.

The [registered gate](../registrations/S02-local-dependency-repair.md) tests correctness and diagnostic work. All **46 relational-package tests** pass, including 16 constructor/source tests, with scoped strict Clippy and formatting. No comparative time or allocation experiment ran.

## The causal distinction

Aliasing two values without descriptors cannot reveal a constructor. A suspended `f(X)` test therefore needs a notification when its value gains a descriptor, not on every identity merge. In contrast, a repeated-variable pattern can become valid when two unknown inputs become identical. Suppressing all alias wakeups would make that pattern incorrect.

The implementation uses separate descriptor and unresolved-equality subscriptions. A known constructor signature needs no continuing subscription: successful unification preserves it, while a conflicting descriptor fails the branch before token service. Established equality is likewise monotone. Stable capture handles already follow endpoint repair, so a ready match does not need reinspection merely because its captured value acquires another alias.

When classes merge, constructor notifications come only from the previously unknown class that gains a descriptor. Either class can survive the size-based handle merge. Equality subscriptions still wake on endpoint merges. Reinspection replaces subscriptions, and consumption unregisters them. The ready set preserves request order; pending equations still settle before a token is consumed.

## What the tests establish

| Check | Evidence |
|---|---|
| Broad constructor aliases | Before descriptor supply, exactly one registration inspection per request and no consumption. After supply, exactly one further inspection and one token consumption per request. |
| Nested descriptor arrival | Four class-size/orientation cases validate late outer and inner information against independent complete source answers. Aliasing the unknown inner value does not cause reinspection. |
| Repeated-variable equality | Eighteen cases combine zero/one/three constructor layers, identity-only or descriptor updates, mismatch and both merge directions. Complete outputs, aliases, residual requests and unused tokens agree with the scalar source evaluator. |
| Existing source coverage | The 200 repeated/distinct-pattern cases, 2,160 independent constructor/equation configurations, occurs checks, forks and consuming-order tests continue to pass. |
| Subscription ownership | Consumed and permanently mismatched requests have zero subscriptions. Later endpoint repair and descriptor changes do not revisit consumed requests. A fork retaining unresolved requests keeps its subscriptions and independently correct answer while its sibling consumes. |

The existing broad test passed before implementation. The new causal assertion then failed at width eight: descriptor-free merges caused 43 inspections where only eight registrations were permitted. That [failing receipt](s02-local-dependency-repair/red.log) distinguishes the repair from a test that merely recognizes the new data structures.

| Constructor requests | Earlier total inspections | Repaired total inspections |
|---:|---:|---:|
| 1 | 2 | 2 |
| 8 | 51 | 16 |
| 64 | 2,207 | 128 |
| 256 | Not measured in the baseline gate | 512 |

The earlier finite counts include registration, conservative alias rechecks and final activation. The repaired counts include registration and activation. Handle relocation, set maintenance, occurs checks and term matching still perform work; this table does not measure their total cost.

## Equality remains a consequential contrary case

An additional causal probe creates many requests waiting for a common value to equal distinct other values. It then aliases the common value with fresh unknowns that cannot satisfy any waiting pair. Every endpoint merge still wakes all those requests. Width eight produces **72 inspections**, and width 64 produces **4,160**, before any useful equality arrives: `n` registrations plus `n²` rechecks.

The probe also supplies the useful equalities and validates complete source answers, FIFO token identities and subscription release. Its untouched fork retains the independently expected suspended answer. These checks prevent the adverse work count from hiding missed activation or altered source behavior. The exact width-eight/64 probe was added during repair review as an exploratory diagnosis of the equality limit anticipated by the registration; it is not a prospectively timed comparison.

This cost is an implementation question. Indexing the actual awaited value pairs could avoid visiting unrelated requests, but adds relation lookup, endpoint repair and descriptor-transition maintenance. Merely filtering after scanning all watchers could reduce the inspection counter while retaining quadratic discovery. A credible next comparison must count that discovery and maintenance as well as full request inspections.

## Four-package breadth review and next selection

This review counts the reunion identity attribution, stronger-control gate, complete reunion pilot and current dependency repair as four packages since the prior breadth review. Changing the active task does not restart that count.

**Next investigate indexed equality dependencies on these complete consuming-source witnesses.** Compare them with the current endpoint subscriptions and a simple event-filtered control, while charging notification discovery, relation migration, descriptor transitions and full matching. Keep the constructor-only favorable control and late structural/alias adversaries. Independent source answers and subscription ownership remain required.

| Strong alternative | Why it matters | Selection at this boundary |
|---|---|---|
| Broader integrated heads, effects and history | Can establish whether the organization removes service boundaries across more CHR programs | Required. The now-demonstrated equality confound can dominate its consuming-source comparisons, so one bounded causal dependency comparison comes first. Return to broader source execution at that boundary. |
| Repeated dynamic reunion and inferred boundaries | Can broaden economical explicit decomposition beyond the finite checked phase | Required under T077. It needs a new source/ownership argument; the equality witness already supplies a concrete obstacle and ready controls for a distinct architecture. |
| Broader source elimination and direct resource solving | Can avoid work that both execution organizations perform | Required under S06. Compare against broader integration after the dependency boundary; the finite countdown result does not resolve these general mechanisms. |
| More constructor-only precision | Could quantify the size of an already validated work repair | Not selected now. No complete cost claim is being made from that counter, and precision would not settle the newly observed equality discovery cost. |

The further local package is justified by a new, exercised adverse mechanism, not by assuming that the current integrated design deserves adoption. If indexing only moves quadratic work into relationship maintenance, preserve that result and reconsider the complete organization. A successful repair still needs complete lifecycle costs and broader source support. This is the first T072 package since the reunion boundary; it does not reset the sequence's broader coverage obligations.

The current source plan remains restricted to one consuming `take`/`token` rule with a captured-variable equation. Arbitrary bodies, general head combinations, propagation history, explicit scheduling alternatives, preparation/compilation, sustained lifetime, coherent complete architectures and held-out challenges remain unresolved. Reference-interpreter and independent-evaluator implementations are unchanged. The architecture goal stays active.

Evidence: [baseline](s02-local-dependency-repair/baseline.log), [causal failure](s02-local-dependency-repair/red.log), [final tests and diagnostic counts](s02-local-dependency-repair/tests.log), [Clippy](s02-local-dependency-repair/clippy.log), [formatting](s02-local-dependency-repair/format.log), [audit and source hashes](s02-local-dependency-repair/audit.json). Scoped Clippy succeeds; its log also records existing dependency build-script warnings.
