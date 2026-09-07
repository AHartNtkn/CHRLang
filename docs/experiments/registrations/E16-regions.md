# E16 certified-region worker gate

The [cold equation comparison](../results/E16-costs.md) motivates an independent
coarse boundary: retain each certified region's persistent search on its worker and
transfer owned answers instead of individual equation operands and substitutions.
The existing E08 permanent partition certificate and sequential factored engine are
the semantic controls. This gate measures no elapsed-time advantage.

## Experimental contract

Keep the existing predicate/query-variable partition certificate. Coupled predicates
or shared writable query variables stay in one region. Construct each regional Rc
search inside its assigned worker and retain it there; fixed assignment is sufficient.
The owner alone manages accepted regional answers, done/empty flags, product jobs,
renaming, output-name order, exact global deduplication and refutation.

A request identifies its region, unique request ID and positive source-step quantum.
The worker returns ordered regional answers, exhaustion and numeric source/observation
counters. Permit at most one unaccepted request per region plus a total reservation
limit K. A received result retains its reservation until logical acceptance. Reject
unknown or duplicate replies as execution errors. Never use physical completion order
to choose product-job size snapshots or answer admission.

At each logical owner turn, perform at most one deterministic prefetch pass in cyclic
region order from the next selected source region, reserving at most K unfinished
regions. Reply receipt alone cannot start a fresh pass. Preserve the original source/
product alternation and round-robin selection, accepting only the selected region's
reply on a source turn. Product work stays on the owner. Completed but unaccepted
regions remain logically unfinished. Record speculative regional work separately from
accepted source counters and owner turns.

Q=1 must reproduce E08's per-turn full answer sequence, exhaustion, source/product
counters and complete raw multiplicity. Q=8 is a separate explicit regional scheduling
control: one source turn accepts up to eight source steps' ordered answers before the
next product turn. Compare inline and workers under the same Q, rather than attributing
different prefixes to parallelism. This changes experimental scheduling of certified
independent regions, not the source rule selector or adopted language semantics.

Use bounded request/reply channels with a documented submission/drain argument. On
empty-factor refutation or a requested prefix stop, stop issuance, signal cancellation,
drain every accepted request acknowledgement and join. Workers check cancellation
between source steps. Report actual additional work, canceled requests, and unaccepted
results; no shutdown reply enters product observation. A single source step remains
atomic, so this is not bounded wall-time cancellation. Worker panic/channel failure is
an execution error, never an empty region or a logical refutation.

## First implementation and validation sequence

Keep the existing E08 engine as the baseline. Implement a separate owner coordinator
used by inline, one-worker and two-worker modes. Use K=4, with K=1 controls for forced
reservation checks, and Q=1/8. Do not convert Rc to Arc or modify the reference interpreter.
The one-region path must remain semantically correct; a worker control must actually
execute its regional service rather than silently falling back to inline execution.

Before a comparative semantic matrix, focused tests must cover:

- Q1 equality with E08 ordered observations; Q8 worker equality with its inline control.
- Forced reverse completion and two real workers entering regional service together.
- Reservation retention after receipt and duplicate/unknown protocol errors.
- Regional variable freshness across outputs and residuals, repeated output aliases,
  duplicated regional alternatives, and exact product deduplication/raw multiplicity.
- Empty-region refutation beside an infinite producer, and an unfinished region that
  is never treated as empty. Cancellation must account for accepted requests.
- Infinite regional producers returning the tested finite product prefix without
  waiting for exhaustion; prefix stop remains distinct from exhaustion.
- Coupled source rules/shared variables retained by the certificate, single-region
  behavior, and complete cleanup after prefix, refutation and worker failure.

Then register the exact case/configuration matrix using E00, E08 controlled products,
connected applications and unequal regional work, with hand expectations and independent
reference checks where feasible. Freeze source hashes and semantic resource bounds
before running. General fairness, useful speedup, lifetime storage and larger service
quanta require follow-up experiments, not inference from this gate.

## Frozen first matrix

Use the 103 finite E08 fixtures without changing their constructors: 62 exhausted E00
cases, 40 product cases (k=1/2/4/8, work=0/1/8/64/256, noise=0/16), and mixed-add-infer.
Their constructors now reside in a shared experimental fixture module used by both
probes. Run baseline E08 and Inline/Threads1/Threads2 at Q1 and Q8, all K4: 721 rows.
Each case uses its existing budget multiplied by eight owner turns, stops at exhaustion,
and must return its full hand-derived answer set and raw multiplicity. Directly check
the 62 E00 cases with the independent reference. Large controlled products use their
explicit Cartesian-product expectations and baseline correspondence rather than a
copying-reference cost run. The original finite fixture count must remain 103.

For each case, Q1 modes must match the baseline per-owner-turn answer counts, cumulative
product/source counters and exact ordered answer sequence. Q8 workers must match their
inline Q8 control on the same fields. Final observations must agree across both quanta.
Include committed source applications, factors, raw products, owner/source turns,
products and product-job counts in durable rows. Record request, acceptance and actual
regional-work totals separately when available; cancellation-related actual work may
vary, while logical observations/counters may not. Reproduce the entire matrix once.

Run each complete semantic matrix subprocess with a 300-second wall and 1-GiB address
space bound, retaining partial output/errors. Record host/toolchain/source hashes first.
No timing inference is drawn from this gate. Focused infinite-prefix and protocol tests
are separate evidence, not silently substituted finite matrix cells.


Before the final receipt, also require transport step totals to equal sums of their
accepted/actual regional snapshots, including prefix and refutation shutdown paths.
The initial finite matrix has no unaccepted shutdown requests, so it cannot witness
cancellation. A forced accepted worker request must acknowledge cancellation with
known served steps and unused quantum slots, without logical admission. Keep that
focused evidence separate. The v2 matrix adds the snapshot-sum assertions; its logical
contract and 721 configurations remain unchanged.
