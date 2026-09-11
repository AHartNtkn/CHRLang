# Context checking: a real lifecycle gain, with an adverse case repaired

The graph evaluator repeatedly checks whether a result's required choices are present in the current execution context. Changing that check roughly halves the larger dependency lifecycle cost without adding retained state. It does not close the measured gap to direct execution, and the fastest check depends on the arrangement of keys.

## What changed, and why

The preceding [profile and lifecycle study](S08-traversal-lifecycle.md) identified context searches as substantial production work. We implemented two alternatives to individual ordered-map lookups: walking both maps in order, and seeking across gaps while continuing sequentially through adjacent keys. The same positive inclusion predicate is used at twelve production sites; compatibility and consuming-resource semantics are unchanged. Compile-time features isolate the alternatives; ordinary lookup remains the default pending a qualified selection policy.

We created dense, prefix, late-key and widely separated-key fixtures, with successful inclusion, early and late conflicts, missing keys and empty support. The first alternative exposed a concrete defect in the proposed optimization: it scans irrelevant gaps. The seeking implementation repairs that cause rather than excluding the adverse fixture.

## The operation comparison explains the tradeoff

Ratios below are candidate time divided by lookup time, using the 256-key matching cases. Lower is better. These are operation measurements, not whole-program rankings.

| Required keys | Ordered walk | Seeking across gaps | Meaning |
|---|---:|---:|---|
| All 256 | 0.323 | 0.317 | Sequential traversal avoids repeated tree searches. |
| First two | 0.924 | 0.700 | Both stop early; seeking also has a favorable constant here. |
| Last two | 15.444 | 0.991 | Seeking repairs the large irrelevant-prefix scan. |
| Four widely separated | 10.878 | 1.670 | Repeated range setup still costs more than direct lookup. |

The initial two-method campaign independently found a 14.5× late-key penalty. The three-method confirmation retains that adverse control. Seeking's late-key ratio ranges from 0.945 to 1.131; this does not establish a 10% gain. Its separated-key penalty is consistent across all nine pairs (1.474–1.865). A selection policy must earn its dispatch cost and preserve these cases.

## Whole lifecycles improve, but not uniformly

The registered matrix covers two changed queries per preparation, pure and consuming sources, two graph engines, reclamation, immediate and retained consumers, continuing demand and flat exhaustion. All 1,120 sample processes validate complete answers and disposal. All 160 cells reproduce exact allocation phases across repeats, across context algorithms, and against the preceding matched control.

A representative pure continuing case requests 128 answers per query, consumes immediately and does not explicitly reclaim:

| Engine/check | Median complete lifecycle |
|---|---:|
| Dependency / lookup | 65.302 ms |
| Dependency / ordered | 37.258 ms |
| Dependency / seeking | 33.057 ms |
| Template / lookup | 6.055 ms |
| Template / ordered | 5.504 ms |
| Template / seeking | 6.098 ms |
| Direct control | 0.560 ms |

Seeking meets the preregistered gain criterion in 18 of 32 continuing graph comparisons against lookup; ordered does so in 14. The remaining comparisons are unresolved under that criterion, with no qualified regressions. All continuing graph variants remain slower than their matched direct controls. Thus context checking explains a meaningful part of dependency cost, but cannot explain the remaining architecture gap by itself.

Flat cases yield only one qualified seeking gain and two ordered gains out of sixteen each. They do not justify a general small-query policy; the preceding closely interleaved 11,000-lifecycle study remains the relevant noise control. No new RSS claim follows from this campaign. The exact heap audit establishes unchanged requested ownership; compilation and process startup are not included in this endpoint.

## Evidence and next decision

Independent coordinate truth checks all 531,441 assignment pairs per semantic run. Ten frozen semantic/work confirmations cover the two stages; each reproduces 51 key-comparison cases. Ordinary operation sizing contains 918 initial and 1,377 confirmation rows. Default and feature-enabled regression suites pass, including four consumer-pressure tests with seeking enabled. Scoped Clippy passes for the library/tests and both lifecycle runners.

Next, qualify a bounded density-based selection between lookup and seeking, including sizes around its boundary, separated keys, early failure and small contexts. This is a concrete attempt to retain the dense gain while fixing the remaining sparse penalty. It has higher immediate value than changing memo storage because the measured adverse cost is inside this predicate and the repair needs no new retained owner. If dispatch cannot preserve the useful lifecycle gains, retain the conditional result and compare memo-storage attribution with partner planning at that gate. The direct comparison prevents presenting another predicate improvement as sufficient architecture evidence.

This is package two since the full portfolio review. T074 and the research goal remain active. Broader integration, solving, partner planning and complete architecture comparisons remain required work; no direction is rejected by this component result.

Reproduction: run `audit_context_inclusion.py` and `audit_context_lifecycle.py` under `research/chr-reuse/experiments/`. Registrations: [initial gate](../registrations/S08-context-inclusion.md), [seeking repair](../registrations/S08-context-seeking.md), [lifecycle matrix](../registrations/S08-context-lifecycle.md). Frozen sources, commands, raw receipts and machine-readable analyses live in [initial evidence](s08-context-inclusion/analysis.json), [repair evidence](s08-context-seeking/analysis.json) and [lifecycle evidence](s08-context-lifecycle/analysis.json).
