# R02 integrated consuming execution passes its semantic gate

The candidate executes consuming source rules while constructor equality deductions remain pending, and publishes only after complete consistent quiescence. Successful traces pass independent atomic-equation replay. Failed speculation and pending contradictions beside recursive source work produce no answer.

The [registered gate](../registrations/R02-integrated-semantic-gate.md) is implemented in `research/chr-integrated`. Thirteen semantic tests pass in default and counter-free configurations, including 33 rotations of direct/indirect binding-and-consumption inputs and independent workload formula checks. Two runner tests check full observations and actual preparation reuse across all five execution variants. The [validation logs](R02-integrated-pilot/all-tests.log), [counter-free logs](R02-integrated-pilot/off-tests.log) and [review receipt](R02-integrated-pilot/review.md) preserve the gate evidence.

## What is independently established

The test-only replay starts from the original query, validates each recorded ordered occurrence tuple, expands complete bodies and solves their equations with owned syntax substitutions and an occurs check. It never calls candidate, persistent or reference matching/equality routines. It enumerates remaining source tuples for missed applications and compares full outputs/residuals under a joint variable bijection. Surviving occurrence IDs and propagation histories also agree. This directly checks most-general observations, resource multiplicity and complete body accounting.

Separate candidate representation audits inspect all retained constructor descriptions, reconstruct occurrence indexes and report outstanding source/equality/repair obligations. Those audits reuse candidate term inspection and are not the independent semantic oracle. Both layers must pass. Production publication does not call the exhaustive audit or repair a failed audit by rescanning.

The partial-equality witness uses a constructor equation with an early useful deduction and further queued deductions. Its successful application trace both records still-pending equality and passes independent replay. A final constructor clash makes the corresponding speculative run fail; that trace is not required to serialize to a successful atomic-equation run. A recursive source consumer alongside the pending clash still reaches failure within the registered step bound.

Unknown structure, repeated slots, positive constructor guards, nested congruence, mixed kept/removed heads, fresh locals, equality-only cycles, constructor cycles, symbol/arity clashes and nonconfluent source competition have explicit checks. No mandatory source ordering or implicit OR search is introduced.

## Organization and cost responsibilities

Values use union-find classes and constructor descriptors. Constructor interning uses current child representatives; child incidence schedules congruence repair after merges. Source argument indexes use canonical classes, with affected columns migrated when classes join. Parent incidence wakes source occurrences whose nested structure changes. Active occurrences supply pattern information before partner lookup. Distinct occurrence IDs and propagation tuples survive value fusion.

Source and consistency service alternate; equality and descriptor repair also receive alternating service. Entire conjunctive bodies are expanded on firing, so producer consumption cannot discard a hidden body obligation. Cycle rejection checks reachability between the two classes about to merge; the exhaustive graph check belongs to audit. First observation exports directly without diagnostic history. Preparation is immutable shared rules, while query values, indexes, queues and histories are separate owned state.

The candidate retains value/descriptor records and occurrence slots until query disposal. It currently maintains columns/incidence even for output-only predicates, unlike the dedicated control's no-head shortcut. These are implementation costs to measure, not semantic necessities or evidence of an inherent lower bound.

The gate establishes finite successful source correspondence and the stated bounded failure/progress witnesses. It does not establish general termination, OR/search, arbitrary guard stability or architectural performance. The [first cost pilot](R02-integrated-cost-pilot.md) has its own narrower exercised-work boundary.
