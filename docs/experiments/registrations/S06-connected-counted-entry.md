# Connected finite projection: counted source gate

Registered before implementation and runs. T076 asks whether connected hidden
coordinates can be eliminated while retaining the number of source answers.
The numeric projection engine already supports weights; this experiment repairs
its term adapter rather than creating another solver.

Hypotheses: duplicate domain choices contribute multiplicatively; repeated
predicate checks do not. Shared hidden coordinates preserve output correlation.
Repeated output slots preserve aliases without multiplying answers. Contradiction
produces no answers under both counted and set observation.

Run a deterministic matrix of three variables, domains containing atoms a/b and
lam(a,a), with and without a duplicate a, four connected predicate patterns
(shared hidden inequality, equality plus inequality, repeated inequality,
contradictory equality/inequality), eight visible masks, and ordinary/aliased
output slots. This is 128 sources and 256 observations. Compare every weighted
tuple against independent full domain-choice enumeration. Compile each predicate
as an explicit finite truth-table CHR rule and run the existing independent scalar
interpreter; compare complete residual-free source answer bags and reference sets.
Use each prepared region for unrestricted and changing visible restrictions.

Controls: set projection, counted projection, independent enumeration, scalar CHR
execution and unchanged reference interpreter. Repeated rows in a predicate table
are predicates, not source alternatives; source domain duplicates are alternatives.
The output is a bag, not a claim about scheduler order or cancellation latency.
Those require an ordered source transport experiment after this gate.

Bounds: 100,000 projection operations per call; 1,000,000 scalar/reference steps
per source; three-variable matrix, no random seeds or timing repetitions. Run
semantic tests with default metrics and no default features, then scoped Clippy.
A mismatch blocks cost measurements and must be repaired. Passing licenses a
connected lifecycle pilot, not a performance verdict. Record exact work on an
additional connected star and dense control using the existing numeric engine;
charge local relation construction separately before claiming eliminated work.
