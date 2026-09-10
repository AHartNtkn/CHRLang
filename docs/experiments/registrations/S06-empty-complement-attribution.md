# Avoid copying a state when no unmatched assignments exist

The completed finite lifecycle pilot requests 16.39 MB for finite solving on four
unselective queries, versus 9.24 MB for specialization. Private solving accounts
for 12.18 MB. Inspection finds a specific avoidable allocation: matching a
singleton domain clones the complete state, excludes its only value, then drops
the empty alternative. The selective finite/conditional allocation differences
are also small enough that this correction could change their ordering.

Change only that clone: test domain cardinality before copying. A matching
condition already establishes that its value belongs to the domain. If there is
only one value, the unmatched region is empty. Keep selection, weights, source
priority, work limits, service events and caller execution unchanged. Preserve
original sources and binaries. Existing independent phase/bridge tests must pass;
work signatures and full native preflight answers must agree before costs count.

Use the frozen ascending baseline binaries from `s06-finite-lifecycle-extended`
and freshly built ascending candidate binaries with exactly the same flags. For
each of four families (oldest, newest, all, duplicates), compare finite mode on:
(0,1,0,none,false,0), (4,4,0,none,false,2),
(4,4,all,none,false,2), (8,1,0,none,false,2), with tuple meanings inherited from
the lifecycle registration. Add unchanged Scan calibrations for oldest and all
at (4,4,0,none,false,2). These are 18 cases and 36 version/case cells.

Run two allocation processes per cell, seed 7942 (72 runs), followed by five
ordinary-allocator processes per cell, seed 7943 (180 runs). Use unchanged
1 GiB/60 CPU-second/75 wall-second process bounds and the corrected eight-million
runner call bound. Freeze source/binaries before runs. Require exact allocation
replays, work equivalence, full endpoint/answer preflight and owner restoration.
The Scan allocation calibrations must remain identical.

The causal allocation contrast is this single source change. Report requested
traffic/peak and phase location pointwise, and five-sample timing medians/ranges
without significance claims. Retain an unselective disadvantage if it survives;
this repair is not a reason to keep tuning the prototype automatically. Reassess
native graph/connected feasibility after this attribution and the breadth review.
