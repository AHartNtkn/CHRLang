# Repeated separation preserves answers under checked ownership

A branch can separate again after a joining rule finishes, without losing its earlier choices, consumed resources or rule history. The new engine passes 30 finite source configurations against existing controls. This establishes feasibility under the fixed-owner rule contract; its total cost remains unmeasured.

## What this answers

The earlier reunion engine separated only the initial private phase. That cannot establish whether a computation can repeatedly alternate between private work and coupled execution. This experiment adds that behavior to the same prepared-rule representation and compares it with the initial-phase engine and ordinary Copy. The independent scalar evaluator remains unchanged.

Separation requires more than different variable names. Two owners may reach the same unresolved variable through aliases or nested terms. The new check follows resolved live terms and permits separation only when their unresolved roots are disjoint. A shared ground value permits separation. A shared unknown keeps the branch coupled until a later completed body makes separation eligible.

## Evidence and its limits

| Question | Direct evidence | Interpretation |
|---|---|---|
| Does separation really repeat? | 24 configurations: zero through three additional rounds, private depths 0/1/4, distinct or duplicate choices. Diagnostic assertions require multiple epochs for additional rounds. | Repeated execution is exercised, including frequent reconnection with little private work. |
| Are choices and resources preserved? | All 24 agree on complete raw answers with scalar, Copy and initial-phase reunion. Expected answer counts are 2 or 4 raised to the number of rounds. Each round consumes both ready occurrences and leaves a record of its choice. | Products preserve branch correlation and duplicate derivations in this family. Competing next-round and terminal consumers follow source priority. |
| Does propagation history survive? | Kept stamps create exactly one fresh alias pair per owner across every repeated-round answer. Direct transport tests preserve an inherited cross-owner history tuple. | Existing history must survive both partition and reunion; fresh identities require separate relocation. |
| Can late equality change eligibility? | Two source cases cover initially separate and shared unknowns. Both agree with scalar and Copy; diagnostics witness separation after grounding and coupled service for the shared case. | Eligibility follows current resolved ownership, not initial names. |
| Can prepared rules serve changed queries? | Four changed-query runs agree with Copy; the existing test also checks those queries against scalar. | Repeated execution uses a reusable prepared object. |
| Can unfinished work block publication or leak ownership? | A finite sibling is published within 1,000 service calls while another private branch continues. Direct weak-owner checks show cancellation releases saved states and the prepared owner. | Bounded service and these cancellation owners pass; sustained lifetime is still unmeasured. |

These are 30 finite repeated-engine configurations, plus the ongoing service case and three direct ownership/transport tests. Finite repeated service has a 200,000-call cutoff; the existing Copy helper has a 100,000-call cutoff. Every finite case exhausted. A cutoff would be unfinished evidence, not an empty answer set. Existing initial-phase tests also pass.

The full restoration test targets pass with 28 tests in the ordinary build and 29 with diagnostics. Scoped strict Clippy passes. [Validation logs](s04-repeated-reunion-gate/default.log), [diagnostic logs](s04-repeated-reunion-gate/diagnostic.log) and [Clippy output](s04-repeated-reunion-gate/clippy.log) record the commands' results. The [registration](../registrations/S04-repeated-reunion-gate.md) states the contract and intended checks before implementation.

## What the implementation must pay for

Each separation traverses resolved live terms. Each private component retains a copy of the inherited binding map and rule history, and owns its portion of live occurrences. Reunion preserves all pre-boundary identities, relocates only newly allocated variables and occurrences, and checks overlapping bindings for agreement. A separate queue serves each branch's epochs; alternatives from different coupled branches never share a product table.

Those responsibilities are necessary for this implementation's correctness and potentially expensive. No native timing, allocation matrix, work-saving claim or architecture ranking follows from this gate. Ground independence alone does not establish that a split is worthwhile. Fresh ranges and histories can grow even when the next private phase does little work.

## Next decision: charge repeated separation before generalizing it

Select a bounded allocation/ownership comparison next: ordinary Copy, initial-phase reunion and repeated reunion on the same sources. Include depth-zero frequent reconnection, substantive private work, late links, increasing inherited history and changed-query reuse. Count eligibility traversals and private/coupled work separately from allocation runs. Charge preparation, query setup, execution, observation and disposal, and prove cancellation and prepared-owner baselines before ordinary timing.

This is more immediately informative than another semantic variation: it can show whether repeated splitting saves enough work to cover the retained contexts and reunion machinery. The strongest ready alternatives are broader integrated heads/history under T072 and resource-aware lowering under T076. Both remain required. One bounded ownership/cost package is justified now because the new mechanism is source-qualified and its dominant potential liability is explicit; review that result against those alternatives before selecting another refinement.

Automatic owner inference, adaptive splitting, general continuation extraction, broader consuming schedules, sustained lifetime and complete architectures remain unresolved. The research goal remains active. This result advances repeated reunion within S04; it does not discharge the rest of that stage or the 57-question sequence.
