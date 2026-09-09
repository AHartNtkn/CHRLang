# Gate recursive-update lifecycle accounting before timing

Validate the new recursive-update lifecycle runner with independent complete answers, exact allocation replay and cancellation. Meter elapsed times are not comparative timing evidence.

Ten families: pass, unary, nested, open, late, malformed, choice, multi, multi-choice, fail. Recursive updates are passthrough, one f constructor, or a nested pair/f/tag expression. Open and malformed tails remain residual; late tails are bound by an ordinary later rule. Multiple calls compete for one optional token. Choice families retain independent terminal alternatives. Fail uses a ground seed incompatible with the supplied ground result, including at zero depth.

Cross first-call depth 0/1/16, query count 1/4 and resource absent/present. Queries alternate depth n/n+1 and initial occurrence order; a second recursive call has one additional step. Compare five build/mode configurations: feature-off original and sealed; feature-on original, sealed and contracted. All use Global+Indexed execution. These are allocation controls, not five new runtimes.

Run all 600 cells twice with counters disabled and alloc-meter enabled: 1,200 allocation processes. Run cancellation at zero/one ticks on the first of two depth16 queries, resources present, all ten families and five configurations: 100 cancellation processes. Total 1,300 plus one meter self-check per binary. Each process uses the first available CPU, 60 seconds wall/CPU, 1 GiB address space and two-million-step source/scalar bounds. Preserve failures and investigate before continuing.

Preparation reuses source-derived rules across changed queries. Measure source construction, preparation, input construction, setup, full execution/observation, engine/answer disposal and prepared disposal. Independent scalar expectations are computed before these intervals, checked afterward, and actual query fixtures must agree. Require zero/one/two/four expected complete alternatives according to the source, complete raw residuals/aliases, exact per-phase allocation replay and restoration of query/prepared requested-live bytes. Cancellation delivers only valid answers and must not affect the next query.

Freeze source, manifests, driver, toolchain and both binary hashes. Review source ownership and primary endpoints before timing registration. Allocation counts describe requested heap demand, not RSS or speed. Source changes, longer lifetimes, native compilation, larger sizes and timing repetitions are later experiments with their own registrations.
