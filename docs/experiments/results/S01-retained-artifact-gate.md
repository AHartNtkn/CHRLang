# Retained matching now shares the native experiment's source boundary

Indexed rediscovery, eager retention and demand subscriptions now run the same subscription source and changing queries as the generated and prepared-plan controls. All ten modes pass complete independent answer checks under ordinary linking and ThinLTO. This closes a control gap before the lifecycle comparison; it establishes no performance ordering.

## The comparison is now executable

The shared artifact runner uses the existing retained source executor and join implementations. Preparation validates the exact supported ruleset. The generic artifact selects its indexed, eager or subscribed policy at runtime, with no per-ruleset native compilation. Generated configurations still use separately compiled source artifacts.

The common runner owns source construction, preparation, query setup, execution, observation and disposal intervals for every mode. Retained preparation consumes and releases the source AST inside the preparation interval, matching the compiled control's ownership boundary. The retained observer's temporary answer vector is converted to the deterministic optional-answer result inside observation, including the vector's disposal. More than one answer is rejected rather than silently discarding results.

The adapter uses enums to hold prepared states and engines. It introduces no additional heap box for selecting an engine; its dispatch and representation participate in the common runner. The experimental runtime includes both implementations. Artifact size therefore remains a property of this experimental build, not a minimal production architecture estimate.

## Evidence

Under the [prospective registration](../registrations/S01-retained-artifact-gate.md), the matrix runs subscription sizes zero, eight and 32 with four changing queries in each of ten modes and two compiler configurations. All 60 processes complete, validating 240 queries against independent scalar source execution. Query and lifecycle sums reconcile, and the fixed runtime hashes remain unchanged through artifact compilation.

Six directed checks reject unsupported retained source families, native artifacts used as retained modes, and artifacts paired with the wrong ruleset. Each rejects before query output. Before integration, the retained mode failed as unsupported; the receipt preserves that behavior check. This new boundary makes no claim that the existing retained engine supports arbitrary source programs.

Sixteen existing tests pass with counters disabled, covering independent native artifacts, join maintenance and source execution. Their source cases include consuming competition, duplicate demands, structural aliases, delayed bindings, missing resources, retirement/reopening, changed preparation, cancellation and failure. These establish semantic coverage beyond the deterministic artifact matrix; they do not measure cancellation cost. Strict emitter/library Clippy also passes.

The [validation receipt](s01-retained-artifact-gate/validation.json), [raw artifact summary](s01-retained-artifact-gate/summary.json) and [source freeze](s01-retained-artifact-gate/source-freeze.json) preserve the run and source boundaries. Earlier gate freezes now identify their verified source commits, so future runner changes do not make their evidence ambiguous.

## Next cost question

The pilot can now compare discovering matches, retaining all matching combinations, subscribing to live demands and generating execution over one source. The prior low-yield and consumption studies identify favorable and adverse retention regimes; the shared artifact query alone does not cover all of them. Carry source-equivalent witnesses from those studies into the registered comparison.

Separate allocation diagnostics must account for the actual installed update plans and retained structures. The existing experimental library already exposes the requested-allocation meter, so it can be reused. Completed-query disposal is measured here; cancellation and artifact-lifetime costs still need their explicit accounting boundaries. The current ordinary runner must not label a metered build as ordinary allocation when that diagnostic path is introduced.

T070 remains active for these accounting prerequisites and the complete cost contrast. The evidence supports proceeding to measurement, not selecting native or retained matching. Direct pull-tabbing and derivation reuse remain the strongest distinct ready alternative at the next selection boundary.
