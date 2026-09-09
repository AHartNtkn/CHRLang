# Partial joins avoid failed-prefix products, while successful prefixes retain extra state

Partial joins remove the full-tuple cache's cold-prefix product: at width 64, both scanning and partial joins make 64 head checks, while full tuples make 4096. All 70 source configurations agree with independent controls. Dense successful prefixes still require a product and retain extra prefix state, so complete costs remain necessary.

## What was implemented

Partial joins use the same equality graph, tuple identity, history and source-order policy as the scanning and full-tuple controls. They register first-head occurrences, then extend only prefixes that have actually matched. A failed prefix subscribes to the missing constructor/equality information without enumerating later partners.

A successful prefix retains its captured handle environment. Extending it checks only the next head, because established matches and captured handles remain valid under consistent additional equality. Prefixes accept newly posted partner occurrences after the complete body. Full matches enter the same ordered ready map as full-tuple activation.

Consumption invalidates every prefix and completed match containing the consumed occurrence. A surviving kept prefix can accept another partner after a child match fires; history belongs to complete ordered tuples, not to that prefix. Tests check reciprocal occurrence/subscription ownership and the requirement that each longer cached entry has a retained successful parent.

## Independent semantic evidence

All 59 previous configurations now pass in all three graph modes, against scalar evaluation and conventional Scan/Indexed execution. Eight new configurations exercise three-head rules, initially known or late first-head information, newly posted partners, and consumption of a suspended prefix. Three dense-prefix configurations add an adverse representation case. Together these are 70 finite source configurations.

The three-head cases require two distinct new partner occurrences to fire through a surviving kept prefix, or zero firings when the prefix was consumed first. Complete raw answers also verify fresh aliases, duplicate occurrence multiplicity, propagation history, source/occurrence order and body scheduling. Every work case passes with metrics disabled. All runs finish within the registered 200,000-call source bound.

## Work and retained entries at width 64

| Family | Scanning head checks | Full-tuple head checks | Partial-join head checks | Full-tuple peak entries | Partial-join peak entries |
|---|---:|---:|---:|---:|---:|
| Sparse descriptor update | 4388 | 197 | 197 | 67 | 67 |
| Broad unknown aliases then grounding | 8451 | 386 | 386 | 130 | 130 |
| Nested repeated-variable equality | 4388 | 197 | 197 | 67 | 67 |
| Cold first-head prefix | 64 | 4096 | 64 | 4096 | 64 |
| Every first head matches; second heads await constructors | 4160 | 8192 | 4160 | 4096 | 4160 |

Partial joins avoid other candidate enumeration even when head counts tie. The sparse case visits 725 fact entries versus 4751 for full tuples and 9397 for scanning. The broad case visits 5198 versus 9098 and 44267 respectively. These are counted operations, not time or allocated bytes; prefix-map iteration, environment ownership and cache maintenance still need lifecycle accounting.

Dense prefixes expose a real tradeoff. At widths 4/16/64, partial joins retain N successful prefixes plus N² suspended extensions, while full tuples retain N² entries. Scanning makes the same N+N² head checks as partial joins on this one-shot source and retains no join cache. Partial joins prevent the cold-prefix product; they do not eliminate products when earlier heads actually match.

Broad equality repair also remains. Both cached modes visit 2143 subscriptions and 2144 changed handles, admitting 64 wakeups at width 64. Partial joins add prefix registration: 322 entries registered across the broad query versus 258 for full tuples. These retained/cumulative responsibilities must not disappear behind a head-check reduction.

## Validation and attribution

The [work audit](s02-partial-join-gate/audit.json) covers 45 mode/family/width cells and verifies exact agreement with all 24 earlier scanning/full-tuple controls. It checks the N cold-prefix and N+N² dense-prefix counts at every width. The [source log](s02-partial-join-gate/source.log), [full package log](s02-partial-join-gate/package.log), [Clippy log](s02-partial-join-gate/clippy.log) and [registration](../registrations/S02-partial-join-gate.md) retain the evidence. The reference interpreter and conventional controls are unchanged.

This is a source/work gate. No allocation, timing, compilation-inclusive or architecture ranking follows. Guarded rules, disjunctive/contextual search ownership and broader lifetime remain unresolved. Successful prefixes add an explicit environment owner; this experiment does not establish the minimum complexity of implementing them.

## Next decision: charge the three general matching organizations

Select a bounded allocation and ownership comparison of scanning, full tuples and partial joins, with conventional scanning/indexing controls on identical source queries. Include cold and dense prefixes, sparse updates, broad repair, three-head consumption and changed-query preparation reuse. Measure preparation, query input/setup, complete execution with the API's observation boundary, cancellation and all disposal owners. Keep work diagnostics and allocation builds separate. Establish owner baselines before ordinary timing.

The comparison can now be representative: it includes a competent way to avoid the witnessed cold-prefix product and an adverse regime where caching adds state. This is more informative than another local work-count refinement. It is the third package in the current broader integrated-matching cycle; review breadth after the ownership package against resource-aware source lowering and adaptive reunion. Those investigations remain required, and no current candidate has architectural priority by incumbency.

The research goal remains active. The evidence narrows how general matching should be compared; it does not choose the language architecture.
