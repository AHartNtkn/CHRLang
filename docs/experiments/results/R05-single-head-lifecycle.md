# R05 scheduling-preserving specialization

Checked single-consumed-head specialization is executable and improves lifecycle medians in 15 of the 16 registered workload/lifetime comparisons. Nine have separated observed timing ranges. It requires no new source-language restriction: eligibility is inferred across the complete ruleset, and specialization preserves source scheduling. This does not establish eager region evaluation or a universal architecture preference.

## Eligibility and responsibility

Every head use of a selected predicate, identified by name and arity, must be the sole removed head of its rule with no kept heads. Body calls do not disqualify it. Multiple constructor cases, repeated variables, equality guards, recursion and nonground calls are accepted. A propagation or multihead use makes that predicate ineligible. Checked requests reject unknown or ineligible targets; inferred selection chooses eligible predicates. Other predicates are intentional ordinary parts of the mixed engine.

The prepared path compiles head and guard tests into flat register instructions. It walks the same ordered indexed or predicate bucket without constructing a generic tuple cursor, partner pools or history keys. Tests use the existing nonbinding equality/constructor primitives. Committed applications retain source rule order, occurrence order, exact removal, fresh-variable/body ownership and normal equality service. Only selected single-consuming rules omit history. Search resets after an application, so equality can make an earlier skipped occurrence eligible again. The immutable plans are shared across starts, templates and OR forks.

The experimental path requires Global policy and explicitly rejects Active. Both Scan and Indexed are implemented; the pilot uses Indexed. Selected bodies use existing prepared body/kernel work. This is prepared instruction dispatch, not native code generation. Source patterns, indexes, occurrence identity, equality, explicit branch search and full observation remain present. Generic patterns needed by ordinary rules and indexed key selection remain part of preparation. Eliminating cursor/history work is narrower than eliminating the entire CHR engine.

## Independent semantic and mechanism gate

Six new integration tests compare full `(answer, rule-and-occurrence trace)` multisets with ordinary execution and compare source-rule sequences/full answers with an independent scalar oracle. They cover eight mixed/nonground examples, 72 guard/constructor/order configurations, five recursive choice sizes and two guard-local cases. They also cover twelve generated-program fixtures, checked eligibility at different arities, rejected external propagation/multihead uses, branch cloning and prepared-query leases. All 40 compiled-package tests pass with default and disabled counters.

Mechanism checks require selected applications to have zero generic cursor steps, history checks and retained history. A prepared template remains usable after its external prepared owner is dropped. Independent review found no concrete eligibility, ordering, ownership or nonbinding defect. The earlier [boundary witnesses](R05-region-boundaries.md) remain the reason this path preserves arbitration rather than eagerly executing a predicate merely because its definition is unique.

## Registered cost evidence

The [prospective registration](../registrations/R05-single-head-lifecycle.md) compares the specialized and ordinary generic global indexed paths on identical programs and queries. All 256 processes completed: 640 queries and 14,240 raw answers independently validated. There are 160 primary ordinary-allocator/counter-free processes; warmups, allocation and work runs are separate. [Raw results](r05-single-head-lifecycle/runs.jsonl), [summary](r05-single-head-lifecycle/summary.json), [source/build metadata](r05-single-head-lifecycle/metadata.json) and validation receipts accompany the report.

Cold single-query median backend lifecycle, milliseconds including preparation, setup, complete execution/observation, engine disposal and prepared disposal:

| Family | Size | Specialized | Ordinary | Ratio |
|---|---:|---:|---:|---:|
| No OR |16|0.087|0.086|1.014|
| No OR |64|0.356|0.393|0.905|
| Opaque choices |16|0.923|0.987|0.935|
| Opaque choices |64|5.658|6.324|0.895|
| Discrimination |16|2.333|2.785|0.838|
| Discrimination |64|10.631|12.348|0.861|
| Recursive output |4|0.198|0.231|0.857|
| Recursive output |6|0.727|0.793|0.916|

Across cold/reuse cells, the fifteen favorable median ratios range from 0.789 to 0.935. The smallest cold no-OR case has overlapping ranges and no demonstrated gain; specialized preparation there is about 19 microseconds versus 13. Other overlapping ranges occur in discrimination16 reuse, output6 cold/reuse, no-OR64 reuse and opaque choices16 cold/64 reuse. Those cells remain unresolved as timing rankings; outliers are retained. The nine separated cells show that eliminating the machinery can matter in complete paths, without implying benefit for every invocation.

Source application and fork counts match exactly in paired work diagnostics. Every specialized application is counted on the direct path; generic cursor/history checks are zero. Requested allocation is lower in every cell. At discrimination64 cold it falls from 7.076 MB to 5.589 MB with 1,071 applications on both paths. No-OR64 falls from 267 KB to 244 KB with 65 applications. This is allocation traffic, not RSS. Additional plan/eligibility preparation is charged, so no omitted preparation stage explains the observed lifecycle gain.

The existing measurement limits remain: generated input syntax is excluded identically, execution and observation time are combined, all batch answers remain retained through backend disposal, and answer disposal is measured after independent validation. Including that separately recorded disposal does not reverse the separated comparisons. Native compilation cost and a measured mixed-predicate workload distribution are not established.

## Architecture and language disposition

T037 is complete at the selected scheduling-preserving contract. Inferred or checked optional selection can obtain these savings without mandatory language restrictions. Requiring every program to satisfy single-head eligibility would exclude useful multihead/propagation behavior without being necessary for this optimization. This result therefore does not justify that language change. Eager region elimination remains a separate certificate question involving interference, termination, progress and full multiplicity; its gate is not implied by these results.

Select T038, a bounded R07 evidence/responsibility/uncertainty checkpoint before another implementation. The portfolio now has viable dedicated, integrated, conditional, finite-solver and statically specialized paths, but no consistent cross-architecture comparison. The checkpoint must identify semantic responsibilities versus prototype structures, reconcile stronger controls, and choose a concrete next investigation by decision value. It is not final closure.

A same-freeze conditional-versus-specialized output comparison is the strongest ready narrow alternative. Corrected conditional output6 was 0.639 ms in its prior pilot; specialized explicit output6 is 0.727 ms here, but different freezes/sessions cannot establish that ranking. Mixed phases, ongoing streams, compilation amortization and broader eligibility may matter more than resolving that narrow cell. T038 must assess those possibilities explicitly rather than infer a transitive winner or commission every possible test.

Workspace tests, strict Clippy, counter-free compiled tests, allocation-runner Clippy and formatting pass. Source hashes were unchanged during the pilot. The broad goal remains active.
