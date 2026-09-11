# Ordered contraction of connected hidden choices

Investigate a concrete alternative to replaying every source choice: for a finite
unequal-name star, condition hidden domains on each visible value, lazily enumerate
only satisfying choices, and merge those streams by their original choice indices.
The prepared projection supplies weighted tuples. The ordering plan retains domain
occurrence indices (including duplicates), not a trace of failed execution.

Use balanced four-way source choices with uniform leaf work so FIFO successful
completion follows choice-index order. Verify that premise against actual source
execution. Instrument source outputs with choice indices to distinguish duplicate
alternatives; these are query observations, not additional consuming rules. Strip
these indices when comparing caller answers. Keep the independent reference
interpreter unchanged. A separate scalar run checks complete raw bags.

Matrix:2/4 coordinates, distinct [a,b,c,d] or duplicate [a,b,c,a] domain,
forward/reverse domain order, first/last visible coordinate, three competing-token
priorities, plain/aliased visible outputs:96 cases. All hidden coordinates differ
from the visible coordinate. Source rules include propagation history and ordinary
competing token consumption. Source bounds:1000000 steps per run; preparation
100000 row work. Run default and counter-free checks once each.

Require raw ordered caller answers, duplicates, complete exhaustion and cancellation
after each answer to agree. Preserve held results across iterator/preparation
release and restart. Count admitted assignments and source Cartesian assignments;
these are algorithmic work counts, not timing. Internal step-budget delivery is a
separate observation contract: contraction need not retain each interpreter step,
and no language decision follows from this fixture. Report actual differences.

If this works, challenge unequal branch lengths before timing complete caller
sessions. If it fails, fix the ordering or binding mechanism rather than narrowing
answer checks. Broader call reuse remains the strongest ready alternative; this
experiment has priority because it tests an actual way to save hidden search work
while retaining ordinary consuming answers, unlike replaying the whole source.

## Unequal-length challenge, registered after the96-case result

Uniform branches match all ordered caller answers:4800 admitted assignments from
13056 source assignments. Cross the matrix with zero/three extra pending True
operations for odd-index alternatives at each coordinate (192 total cases).
These operations change source completion lengths without changing solutions or
caller bindings. First challenge the index-only merge. If it fails, order admitted
assignments by total branch work, then source indices; test the resulting order
against actual FIFO source execution. Use a lazy Cartesian best-first frontier
with duplicate suppression if mixed-radix traversal no longer preserves this key.
Charge its retained states in subsequent lifecycle costs; no constant-space claim.
