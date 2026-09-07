# E15 resource pilot and session measurement gate

All twelve exploratory cells passed full-answer/raw/exhaustion checks. This sizes
resources; one observation per cell does not establish comparative performance.
Raw observations: [E15-cost-pilot.jsonl](E15-cost-pilot.jsonl).

| Case | Service | Untraced whole child (s) | Traced whole child (s) | Peak traced bytes |
| --- | --- | ---: | ---: | ---: |
| opaque-64 | direct | 0.043 | 0.074 | 216,189 |
| opaque-64 | scan | 0.232 | 1.348 | 1,072,114 |
| opaque-64 | count | 0.234 | 1.353 | 1,082,767 |
| app-sk-ignored-hole | direct | 0.080 | 0.273 | 127,982 |
| app-sk-ignored-hole | scan | 24.832 | 162.342 | 28,409,032 |
| app-sk-ignored-hole | count | 25.483 | 161.289 | 28,470,887 |

This supports a 1,800-second/1-GiB child bound for three-query sessions with the same
20-million-action per-query cap. The ceiling remains a measurement resource bound,
not evidence for excluding any workload. Counted actions alone do not justify a
runtime claim: the untraced pilot does not show a counted-status improvement.

The confirmatory harness separates oracle data before timed source decoding,
includes dropping search references in release timing, and records validation memory
separately. The pilot predates those boundary corrections; do not pool its phase
measurements with the registered confirmation. The largest pilot memory is an
evaluation peak, unaffected by the release/validation labeling issue.

Validation: 27 scheduling tests and 22 net tests pass. The session test exercises
three actual queries per fresh child for each service, including a shared nonground
residual variable; all observations and action counts agree across queries. Warm-up
ordering is separately checked. No candidate or reference semantics changed.

The [registered comparison](../registrations/E15.md) has 135 configurations, each
with one warm-up plus five timed sessions and two separate traced sessions. Every
session has three fresh queries. [Manifest](E15-cost-manifest.json) records the
confirmatory harness/runtime and resource limits. Reproduce the pilot resource probe
with `PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/pilot_net_measurements.py
/tmp/chr-e15-cases.jsonl`; current phase accounting follows the corrections above.

T015 remains active. The repeated comparison is pending; heterogeneous operation
sharing, other E15 compositions and all other open portfolio questions remain open.
