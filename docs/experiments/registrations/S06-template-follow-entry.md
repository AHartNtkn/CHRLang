# Template call-following ablation

Register before implementation/runs: keep template instantiation, ground-edge sharing and per-query keys, but set the maximum followed source calls to zero. Compare this with the existing limit of 64. This isolates call contraction; it is not a proposed language restriction or a completed scheduling repair.

Hypothesis: the existing scarce-token winner change is caused by call contraction, even without choices. The zero-follow template must match the scalar short-caller winner; the 64-follow control retains the long-caller witness. Failure requires investigation.

Extend the existing 72-query matched-carrier gate with zero-follow templates (432 executor searches total), retaining independent scalar/denotation checks, alias/failure/consumption/depth/insertion controls, 200,000-tick bounds and prepared reuse. Run the full suspended-source suite and demand unit tests. Add a direct derivation check that limit zero follows no calls and limit one follows exactly one on a finite chain. No timing conclusions; this changes the measured representation, so future cost runs require fresh registration and ownership qualification.

If zero-follow preserves the winner, next investigate preserving source service points during contraction, using the existing carrier scheduler as the explicit control. Keep choice timing and retained template costs independent questions.
