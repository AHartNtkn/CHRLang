# Next: test whether conditional discovery can avoid unnecessary tuples

T078 resumes with conditional discovery because branch-specific execution still incurs substantial allocation after common traversal counting. The question is whether this reflects necessary representation costs or avoidable discovery work. Existing Scan, contextual eager/demand and source-counting paths remain controls.

Inspect `Discovery` and `discovery_tick` in the conditional engine before selecting a change. The engine fixes candidates for a round and executes applications on their live supports. A replacement must preserve source order, supported contexts, distinct consumed occurrences, equality effects, propagation history and finite-answer progress beside ongoing work. Simply taking the first physical tuple is not a justified implementation for multiple supported contexts.

Establish a bounded source witness where a selective discovery mechanism actually prunes work, alongside dense-support, repeated-history and changing-binding controls. Compare complete independent observations before work or cost claims. Keep counters in separate runs; ordinary primary timings remain counter-free. Charge discovery state construction, retention, updates, preparation, changed queries, complete delivery, cancellation and disposal.

The [resource-count lifecycle](S10-resource-count-lifecycle.md) and broader mixed-source pilots identify relevant branch-specific and ineligible sources. Reuse them without restricting the language's domain to these families. Investigate a consequential failure or adverse result before attributing it to conditional execution itself.

The strongest ready alternative is resumable contextual discovery; the eager and restarting-demand controls expose different costs on history-heavy sources. Conditional discovery goes first because existing measured branch-specific costs can change whether its shared execution is a credible complete competitor. Reconsider the contextual alternative after the first selective source/work gate. Broader T078 architecture composition, T079 language properties and sustained lifetime remain required.
