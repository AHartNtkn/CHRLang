# Resource-sensitive reuse needs more than a larger cache key

Including current resources in the key fixes stale-result reuse in the tested finite phase. It does not make arbitrary call contraction valid. Scheduling, intermediate effects and surviving propagation history independently change source answers.

The next experiment should qualify a checked boundary or preserve execution at effect boundaries before measuring broader call reuse. These results neither reject effectful reuse nor select an architectural implementation.

## What the experiment establishes

**Call arguments alone do not identify an effectful computation.** Across 288 queries, a call-only key produces 252 mismatches against independent complete semantics. A key containing the ordered projected resource region agrees on every query, as does uncached contraction. The region table obtains 240 hits: different caller identities can reuse this phase without retaining the entire caller as its key.

The matrix varies recursive depth, duplicate derivations, caller variable namespace, token values and multiplicity, missing resources, kept facts and unrelated caller tags. Direct whole-source persistent execution also agrees independently on all 288 queries. There are 864 direct/uncached/region comparisons per build; call-only mismatches are recorded separately. This is a finite raw-answer multiset comparison, including aliases and residual multiplicity. It does not establish ordered answer delivery or arbitrary continuation equivalence.

**Correct recognition does not authorize moving effects across caller execution.** Four independent challenges fail with both uncached contraction and region-keyed reuse. Because uncached contraction already fails, these examples implicate the chosen execution boundary rather than cache lookup.

| Source situation | Whole-source result | Contracted result | Requirement exposed |
|---|---|---|---|
| A competing consumer acts between two selected rules | The competitor consumes the token; `stolen` remains and the call cannot finish | The call consumes the token first; `won` remains | Preserve rule priority and resource competition across the whole phase |
| A caller observes an intermediate pulse | `observed` remains after the pulse is consumed | No observation remains | Preserve observable intermediate effects or establish a valid commutation argument |
| A caller supplies a binding before call selection | The `known` rule fires | The `generic` rule fires | Validate the environment at the actual source boundary, including earlier rule eligibility |
| A propagation rule has already processed a surviving fact | One `recorded` occurrence remains | Two `recorded` occurrences remain | Preserve history and occurrence correspondence when resuming; residual constraints alone are insufficient |

Paired controls remove the competing consumer or observer, or place the binding after call selection. Those contractions agree. The history witness uses the same propagation rule and surviving fact; restarting from residual syntax causes the repeat. The [logs](s05-effectful-source-gate/final/primary-run.log) include actual outputs and independent source rule traces for each witness.

**Occurrence order is an additional obligation, with a narrower result here.** Reversing two token occurrences changes which value the caller consumes. The probe's signature-wide region preserves all token occurrences in the positive control and agrees. This establishes that order matters; it does not establish a defect in that positive control or prove general occurrence transport.

## What is implemented?

The experiment is an unchecked contraction probe in test support, not a production optimizer or a new baseline. Existing persistent execution computes an isolated phase over selected initial constraints. The probe caches its owned outcomes, imports fresh variables consistently across bindings and residuals, reconnects every initial caller variable, and resumes the complete caller. The independent scalar evaluator remains unchanged.

Three controls isolate different responsibilities: uncached contraction tests the boundary; call-only caching deliberately omits resources; region caching includes ordered constraints, resource multiplicity and interface size. The probe does not infer which region is valid. Its failures remain executable counterexamples instead of becoming silent eligibility exclusions.

Fresh alias tests distinguish shared and distinct resource interfaces. Held answers remain independently valid after changed queries and table disposal. With phase service budgets of zero and one, interrupted collection returns an error without publishing partial cache results; a subsequent complete query recomputes and a later query reuses that complete result.

Cutoff/reuse is not interruptible streaming cancellation. The initial registration requested an unfinished-caller check; the [follow-through registration](../registrations/S05-effectful-source-followthrough.md) records the actual bounded-collection endpoint and keeps broader cancellation required. There is no incremental caller handle or complete heap-owner account in this probe.

## Evidence and limits

The [initial registration](../registrations/S05-effectful-source-gate.md) predates implementation and runs. A transport test first fails on missing answers, then passes with implemented namespace transport. The final deterministic suite passes ten tests in each of default and engine-metrics-off builds, with strict Clippy passing in both. Each executable has a 60-second wall/CPU limit, 1 GiB address-space limit and 200,000 service calls per full run. Probe hit/computation counts remain enabled in both builds for semantic assertions; no timing comparison uses them.

The [manifest](s05-effectful-source-gate/final/manifest.json) freezes source hashes, executables, commands and receipts. The [driver](../../../research/chr-reuse/experiments/effectful_source_gate.py) refuses receipt replacement. Initial compile and Clippy diagnostics, the initial eight-test run, and the pre-Clippy source snapshot are retained alongside the final evidence. The probe adds no production-library behavior; reference and independent evaluator sources are unchanged.

These runs establish neither speed nor economical keys. Every cache miss currently collects the phase before caller resumption, and every hit clones and transports owned syntax. Complete phase collection may change first-answer latency, cancellation behavior and raw answer order even where multisets agree. The tests do not infer general future dependencies, preserve arbitrary live history, or justify mandatory language restrictions.

## Which investigation comes next, and why?

**Continue T075 with a checked boundary and a resumable alternative.** First test an initial rule-priority prefix with consuming rules, complete head-signature dependencies and explicit interface transport. Check whether absence of surviving propagation history is sufficient for that boundary, and test occurrence order, early failure, suspended resource supply, duplicate derivations and ordered answer delivery. This is an experimental eligibility hypothesis, not an adopted language restriction. Then challenge cases requiring caller interleaving: an alternative must retain current occurrence/history context and resume at effect boundaries rather than executing the entire call atomically.

The strongest ready alternative remains adaptive timing attribution. It could resolve policy comparisons in existing fixtures, but the present source results expose responsibilities that can decide whether broader reuse is possible at all. A checked/resumable comparison can now be targeted at those exact duties instead of implementing a general cache on an invalid boundary. It has higher immediate decision value despite requiring new semantic work.

Native local ownership needs related identity and publication work but also dynamic choices and concurrency qualification. Conditional equality/lifetime repair and incremental projection remain independent alternatives. Reconsider all four at the checked/resumable source gate or a concrete obstruction. This completes one package since the adaptive breadth review; another full breadth review is due within three further packages. Broader effect inference, ordered output, sustained ownership and complete architecture costs remain required.
