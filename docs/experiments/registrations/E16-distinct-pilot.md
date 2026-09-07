# E16 distinct-hole resource pilot

The [first pilot](../results/E16-pilot.md) found that increasing repeated-subtree
size mainly grows representation traffic: eight depth-10 alternatives require only
176 owned pair steps. This follow-up separates that effect from genuinely growing
substitution work. It is exploratory sizing, not a confirmatory ranking.

Build balanced trees at depths 4, 6 and 8 with a different unbound variable at every
leaf, unifying against the corresponding all-a tree. Use eight explicit alternatives
or eight sequential equations. Sequential tasks use disjoint fresh holes; alternatives
reuse local identifiers in separate branch environments. Every answer returns its tag
and first hole, with empty residuals. All eight wide answers and the single chain answer
are independently checked against the reference before cost execution.

Hypothesis: the complete owned pair count grows with the full tree, rather than its
depth. Predict 8 × 2^(d+1) pairs for wide (including eight tag equations), and
8 × (2^(d+1) − 1) + 1 for chain (one tag equation). Larger substitutions also increase
owner installation and storage; no result will attribute all added cost to solving.
The workload tests useful operation granularity under the current complete-MGU interface.
A failure of this prediction requires analysis before interpreting granularity.

Use the same five modes, K=4 for new drivers, lookahead 8, one-slot replies, cold
lifecycle boundaries, output checks and separate System/Meter executables as the first
pilot. Six cases × five modes × two measurement kinds = 60 fresh children. Keep the
30-second process-group watchdog and 1-GiB address-space limit. Freeze source/binary
hashes, actual host and order before execution; do not run concurrent experimental
loads. Record errors and bounds without treating them as logical failures or closure.

Compare committed work across all modes and complete service work at matched limits,
including timing/memory agreement. Inspect pair scaling, construction/search/release
phases, total heap requests, peak and retained bytes, and whether the wide frontier
exposes multiple outstanding tasks. Use the results to choose a repeated cost matrix
or identify a representation/installation bottleneck needing an additional control.
Retain the first pilot's repeated/identity and application cases as contrary evidence;
they remain relevant even if distinct-hole work favors workers.
