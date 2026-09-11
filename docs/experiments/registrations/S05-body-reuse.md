# Selected rule-body reuse

Leave rule selection, guards, consuming heads, propagation history and fresh
allocation in the original live cursor. After a selected application, detach its
single pending instantiated body. Reuse that body's pending-work trace, including
choices, equations and posts, then restore bindings and posted facts before the
next live rule selection. Ordinary source execution cannot match a rule while
that pending body is being processed.

Canonicalize the resolved body and its free-variable aliases as the cache key.
Use the existing trace machinery with a private entry rule whose body cannot
recurse into that entry predicate. All posted source predicates therefore become
residual facts. Skip the entry insertion/application when attaching, since live
selection already consumed that step. Preserve each body source step and merge
completion with the next live rule-selection step, as with phase splicing.

Test observers before, between and after private rules, not just after a priority
prefix. Use the existing recursive source with ordinary, failing, fresh-alias and
binding-before-completion families; depths0/2/4, reversed alternatives,
propagating/consuming observers, three observer positions and input IDs10/1000:
288 cases. Repeat each and cancel at0/1/5 steps. Compare every one-step ordered
answer/exhaustion event with Direct and complete raw bags with the independent
scalar interpreter; require actual trace reuse. Bound every complete run at100000
steps. Test foreign-run ownership and held answers after query disposal.

This is a correctness experiment. Preparation of body-specific traces and lookup
on each rule application are costs to measure after the mechanism works. They
may outweigh saved body work. No general rule-order restriction is adopted.

The first288 cases pass29232 checkpoints. Extend token multiplicity to0/1/2,
producing864 cases. Require repeated complete runs to execute zero additional body
trace steps, beyond requiring replay. Add a focused source using posted predicates
$body and $body_ and a fresh variable shared by both posts and an output constructor;
test existing same-predicate caller facts and two consuming token occurrences.
This checks internal-entry naming, occurrence order and cross-boundary aliases.
