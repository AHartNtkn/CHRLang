# R01: integrated activation and code-generation semantic entry

Status: [semantic entry implemented and checked](../results/R01-semantic-entry.md). The [access/lifetime control](R01-access-lifetime-entry.md) is the next selected implementation before cost registration and source freeze. This entry does not select a production architecture.

## Decision and competing explanations

Does a competent execution core gain chiefly by eliminating generic rule interpretation, avoiding repeated global discovery, or changing neither because representation/effects dominate? The answer determines whether generated activation should be carried into subsequent architectural comparisons and which unresolved cost deserves investigation next.

Four cells use two independent factors:

| | Generic execution | Generated Rust rule/occurrence code |
|---|---|---|
| Global rule scan | G-global | C-global |
| FIFO occurrence activation | G-active | C-active |

Within each row, selection and candidate traversal are identical. The generated/generic contrast isolates actual rule execution form. Across rows, compare the selection architecture with its scheduling consequences, without claiming to isolate scheduling from discovery. Retain the same competent term representation, predicate indexes, equality, body-effect policy, history and observation for this first contrast. Document any unavoidable difference before interpreting results.

A generated ruleset accepts separately supplied runtime queries, including recursion depths not known at generation. Emit actual constructor/repeated-variable tests, fixed rule-local slots, partner loops and guard/body code where appropriate. Query-specific AST unfolding, a generic interpreter wrapped in generated entry names, or hard-coded fixture answers would not test the question. Charge required compilation/preparation later; a semantic gate alone establishes no speed advantage.

## Activation contract

Use a coalesced queue of inserted or changed occurrence IDs, predicate/head-position dispatch and resumable partner traversal. There is no requirement for a globally ordered heap of all enabled tuples. Activation is a permitted experimental policy, not a new source priority rule.

Successful equality publishes affected variable information; failed equations publish no live updates. Dependencies include variables reached through current bindings and constructor arguments. An update during activation retains a dirty flag or newer revision. Consumption cancels obsolete work. Reconsider kept heads as required; restarting traversal with charged history checks is an explicit first implementation choice, not retained-join sharing.

Queue exhaustion is not sufficient while effects, binding updates or relevant activations remain. Source completion must agree with an independent check for remaining enabled applications. Source matching and guards do not instantiate query variables to create a match. Occurrence identity remains distinct from value equality; propagation eligibility concerns rule/occurrence tuples.

## Integrated workloads and independent observations

The main finite families have confluent instances so that different permitted schedules share the same required complete answer:

- Recursive constructor computation through simplification, body equations and fresh locals. Queries vary runtime depth against the same compiled ruleset; expected output structure is independently defined.
- Unique-key chain reachability with propagation and job consumption. Expected reachable nodes, live edges, consumed jobs, final outputs and propagation multiplicity follow from the finite chain, not candidate execution.
- Delayed relational activation on the same kind of finite computation: unknown arguments become equal through justified body equations, including variables nested below constructors and equality guards. The late update must make relevant work discoverable.

Adverse hand cases cover middle-head arrival; repeated predicates with distinct occurrence requirements; duplicate-valued propagation tuples; stale work after consumption; failed equations; pure guard wakeup; fresh body variables; and nonconfluent competing consumers, including an insertion enabling an earlier-listed rule.

For the nonconfluent cases, independently validate each selected application and its effects. Do not generate answers from all rule schedules. Across policies, different legal outcomes are permitted. Within a policy, generic/generated traces must agree. Candidate/generic agreement alone is not an oracle: complete answers and residual aliases/multiplicity must also satisfy independent expectations and source-transition checks.

The first fragment is ordinary CHR without explicit OR and the current pure equality-guard theory. It tests execution-core organization, not full search behavior or all possible guards. Unsupported syntax must be identified, not silently compiled under different semantics. Existing independent reference checks can supplement the analytic oracle, but reference algorithms remain outside the candidate.

## Instrumentation and later measurement

Separate discovery, structural matching, binding reads/updates, dependency maintenance, stale/repeated activation, propagation/history, source effects, preparation, observation and retained storage. Logical effects and physical work are distinct counters. All four modes include the same public answer/lifetime contract.

Record compiler source generation and native build artifacts; runtime preparation is separate from compilation and both enter any eventual amortization claim. Before cost runs, register workload sizes, unrelated rule/store dimensions, queries per prepared ruleset, repetitions, bounds, allocator/timing separation and interpretation criteria. Do not run a large matrix simply to confirm a hand example.

If activation only wins because an inadequate global control lacks comparable indexes, repair that comparison. If generated functions still execute generic matching, the compilation factor is untested. If a policy changes finite observations on a confluent fixture, investigate semantics or the fixture claim before measurement. A correct unfavorable result remains evidence; a semantic defect does not reject an architecture.

## Implementation ownership and selection rationale

One worker owns the bounded candidate/compiler implementation and focused tests. Root owns this registration, independent review and generated experiment freeze. Reuse candidate infrastructure where it makes the controlled factors credible; do not modify or reuse the reference runtime as the candidate algorithm. Exact file/API boundaries are implementation details, not architectural eligibility conditions.

R00 selected this ahead of further class/index integration because access and activation costs affect that comparison too, while the present scalar controls do not establish competent generated execution. Direct finite-relation compilation analysis proceeds independently. No performance result is needed from this gate for another architecture to remain eligible.
