# The relational candidate executes complete source rules

The relational candidate now produces complete answers through its own rule executor. It agrees with an independent owned-syntax evaluator on 46 finite configurations, and the existing integrated engine agrees on the 36 configurations within its no-choice scope. These correctness results make a lifecycle comparison feasible; they do not establish a faster or simpler architecture.

## The boundary being tested

Prepared head plans join constructor facts and identified source occurrences directly. Equality updates repair those same relations. The executor adds source bodies, stable equality guards, propagation history, fresh local values, explicit choice and exact publication to that representation.

This contrasts with the existing integrated engine's recursive pattern inspection and activation machinery. Both already avoid exporting a solved term store for matching. The proposed benefit is unified constructor/source join planning and ownership, not an export cost that the control does not have.

Choice copies the complete interpretation, including resources and propagation history. It therefore supports context-local equality and failure through isolation, while charging copying and retaining separate states. Shared contextual equality and alternative incidence representations remain open S02 mechanisms.

## What was checked

The finite gate uses 23 source configurations, each run with its original query and a changed query containing an additional residual constraint. The same prepared relational rules serve both queries. Complete answers are compared with the independent scalar evaluator using exact joint variable equivalence and residual multiset comparison. Raw answer multiplicity is preserved.

The sources cover constructor-enabled consuming joins in both arrival orders; branching constructor bindings; a propagation guard enabled by later aliasing; kept/removed heads and shared fresh body variables; duplicate alternatives; branch-local failure; finite-tree cycles; contradictory conjunctions; and nonbinding patterns and guards. Twelve further configurations vary store size (1, 4, 16), immediate versus delayed structure, and dense versus low-yield matching.

All 46 relational configurations pass. All 36 no-choice configurations also pass against the existing integrated engine with metrics disabled. The ten choice configurations remain outside that control's declared language support; they are checked against the independent scalar semantics. Neither engine nor the reference interpreter was modified to obtain agreement.

A separate ongoing source forks into endless rule applications and one finite answer. The finite answer publishes within 200 advances, while the frontier remains live. This checks fair service across copied interpretations, not constant-time advancement: a single relational join or equality repair can still do size-dependent work.

The earlier 1,664 head-plan configurations and 1,296 ordered equation-pair cases remain passing. Tests, replay, strict Clippy and formatting pass. The [registration](../registrations/S02-relational-source-gate.md), [source tests](../../../research/chr-relational/tests/source.rs) and [validation receipts](s02-source/validation.json) give the inputs, commands and hashes. This gate contains no comparative timings.

## Responsibilities and limits

The executor chooses rules in source order and candidate tuples in occurrence order. It services one equality deduction before each source advancement, processes pending body effects before discovering the next application, and queues choice interpretations fairly. Exact publication requires no pending effects, no pending equalities and no enabled rule. A later contradiction discards its interpretation.

These tests use sources whose compared observations do not depend on physical scheduling differences. Agreement does not prove equivalence for arbitrary nonconfluent rule sets. Empty-head rules are explicitly rejected. The source tests also do not establish bounded memory for ongoing work: retired rows remain allocated, and history and copied state have measurable lifetime costs.

The present implementation enumerates candidate matches, scans prepared rules when selecting an application and recursively compares guard structure. Those choices may dominate cost despite favorable constructor access. Preparation, selection, index repair, copying, observation and disposal must all participate in the next comparison.

## Next decision and priority

T063 remains active for a prospective ordinary-source lifecycle comparison against the integrated control and competent dedicated-service execution. Use the selective-constructor and dense/low-yield update families established here, plus an ordinary low-overhead control. Register preparation reuse, complete endpoints, adverse cases, repetition and resource bounds before measuring.

A favorable result could justify relational planning as a central organization; an unfavorable result could identify full-join enumeration, incidence maintenance or source selection as the responsible cost. Attribution must distinguish those choices from unavoidable equality and consumption obligations. Contextual copying is not yet a reason to reject shared integration.

The strongest ready alternatives are S01's broader maintained-join comparison and S06's direct relational solving path. Both remain required. The immediate S02 comparison has lower additional implementation cost now that both source paths are available, and can decide whether further investment in this representation is warranted. This is an ordering judgment, not negative evidence about those alternatives. Reassess after the bounded pilot rather than proceeding automatically through relational tuning.
