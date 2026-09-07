# E12 selective failure-proof checking

Checking learned paths in the shared arena avoids the full-projection cost exposed
by the preceding control. It preserves the same proof hits and equation work
savings, and improves the long mechanism probes. Cheap early clashes still expose
checking overhead; application timing remains inconclusive on two repetitions.

Code `cb6cc86`. The [300-run matrix](E12-failure-native.jsonl) and
[replay](E12-failure-native-replay.jsonl) use the same 100 registered cases in
Direct, Learn (owned projection) and Native (selective arena reads) modes. All
pass, every non-time field repeats exactly, and logical steps, failures, raw and
unique answers and maximum frontier agree across modes. Learn and Native also
agree on hits, learned paths, actual equation pairs, occurs visits and executed
transitions. Only the proof access/discovery interface changes.

At K8 D64, every family has 255 reusable failures. Learn projects all 256 equations;
Native projects the first failed equation once for discovery, then performs
1,020 head reads for clash paths or 765 for the occurs path.

| Family | Requested bytes: Direct / Learn / Native | Elapsed microseconds, two runs: Direct / Native |
| --- | ---: | --- |
| Early clash | 161,201 / 1,828,681 / 179,851 | 254, 184 / 313, 210 |
| Late clash | 205,564 / 1,806,276 / 157,956 | 423, 322 / 302, 222 |
| Occurs | 749,693 / 947,473 / 131,218 | 1,210, 840 / 270, 270 |

The late-clash service performs 17,152 pairs in Direct and 67 in either learning
mode. Occurs visits similarly fall from 17,152 to 67. Native therefore separates
useful avoided work from the materialization penalty. The early clash needs only
two pairs per branch; its table lookup, head reads and discovery are not free.
A cold K0 D64 late clash has no hits and requested allocation rises from 67,276
to 74,238 bytes. No universal learning policy follows.

SK duplication evaluation has 113 hits in both learning modes. Native reduces
full projections from 139 to four, pair work from 475 to 115, occurs visits from
424 to 184, and requested allocation from Direct's 404,366 to 353,162 bytes.
Peak live requested storage is slightly higher (174,485 versus 174,133 bytes).
Elapsed observations are 649/673 and 639/374 microseconds for Direct/Native, so
an application speedup is not established. The requested type-synthesis prefix
has no hits; Native performs no full projection and has the same requested
allocation as Direct. Prefix agreement does not establish exhaustive synthesis.

## Semantic boundary and checks

Native reads only the next pending equation, following current variable bindings.
Absent equations, out-of-range children and paths crossing variables return no
observation. A separately implemented path checker verifies matching constructor
ancestors and a clash or nontrivial occurs path. It agrees with the owned-tree
checker on all 52,416 registered small path/input combinations, plus additional
empty/invalid-path cases. Source-cursor tests check bindings established by earlier
equations. A committed-consumer example confirms that a learned path cannot prune
an equation which the winning rule never posts.

On a miss, ordinary unification runs. Only a failed equation's saved cursor is
exported for certificate discovery, and the discovered certificate passes the
independent checker before entering the cache. This is a conservative optimization
of the same source operation; it introduces no rule choice or guard side effect.

The benchmark charges path traversal, constructor-head materialization, saved
cursor clones, miss discovery, scheduling and answer observation. Requested live
storage is not RSS. Two observations support these bounded workload comparisons,
not precise confidence intervals or broad architectural rankings. The preceding
[projection receipt](E12-failure-projection.md) records the initial workload-order
diagnostic and corrected controls.

## Disposition

Structural learned failure has a viable small implementation worth retaining as
an experimental mechanism. It differs from whole-state merging: intended programs
can reuse contradictions without reaching identical complete states. This does
not adopt a production policy. Intermediate-binding witnesses, more general
assumptions, lookup indexing, cache lifetime, longer synthesis sessions and
composition with other sharing mechanisms remain feasible research questions.
Operation-table host integration and cost comparison are independent next work.

Reproduce: build `failure_probe` in release mode and run
`research/chr-reuse/run_failure.py OUTPUT --seed 1221`, then another output with
seed 1222. Workspace tests, Clippy and formatting pass.
