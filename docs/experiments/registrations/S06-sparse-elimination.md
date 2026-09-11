# Sparse compatibility joins for finite elimination

Test whether joining existing factor rows avoids Cartesian assignments during
hidden-variable elimination. Retain the current Cartesian traversal as a control.
Order factors by row count; recursively accept compatible rows, checking domain
membership. Do not add indexes in this first comparison. Duplicate domain choices
retain their weights; filter duplicates remain predicates, not multiplicities.

Before cost comparisons, compare both traversals with independent full assignment
answers in set and counted modes, forward/reverse elimination orders, reordered
visible coordinates and changed restrictions. Include empty domains/filters,
out-of-domain filter rows, duplicate choices and incompatible overlapping rows.
Keep output enumeration unchanged to isolate elimination.

Use equality-star and dense-relation witnesses with domains of size4 and increasing
hidden scope. Record Cartesian assignment visits, successful joined tuples and
relation-row probes separately; they are different units. A 12-coordinate equality
star checks whether sparse traversal can finish within100000 probes when Cartesian
traversal exceeds that assignment bound. This is a work/correctness entry, not a
runtime or complete architectural victory. Whole relation construction, preparation,
output and disposal costs, and source answer order, remain the next comparison.
