# Next E15 discrimination: equality shared across different states

Read-only scout analysis during the registered session timing batch identifies an
open question beyond E12 equation memoization: sharing one advancing equality job
among concurrently waiting, different source contexts, with completed-result reuse
and context projection as separate controls. This is a proposal, not execution
registration or adoption of a language/production contract.

E12-equations already checks resolved-operand alpha keys and caller-local replay,
and shows owned memoization can lose to an immutable shared-arena direct control.
The examined applications have few useful hits. Repeating owned-tree cache hit
counts would add little. The new evidence must concern overlap, bounded scheduling,
fan-out publication, active-entry lifetime and representation-matched costs.

At the next actual source equation, freeze the caller. Resolve both operands under
its complete substitution, jointly rename remaining free representatives, and keep
a bijection back to caller representatives. Solve canonical operands under an empty
substitution. Translate nonidentity result bindings through that bijection, extending
only previously free caller representatives. Do not allocate source variables,
overwrite existing bindings, or merge histories/occurrences/counters. External aliases
then see the patch through the existing substitution. A failed operation kills its
registered caller branches; subsequent continuation failure remains branch-local.

Compare current full-substitution direct service, projected direct, completed memo,
active-only sharing and both. The last four share projection/key/solver/patch code.
For a net follow-up also retain the full-substitution net boundary: projecting the
input can itself shrink construction/readback, so that effect is not a sharing win.
Disable whole-state grouping in the primary comparison; test coexistence separately.

Projection, key canonicalization and solver reuse are distinct expenses. Preserve
immutable physical term sharing in the no-reuse control, include cheap pointer-equal
identity equations and structurally equal separately built operands, and charge all
traversal/key/patch work. Sharing must not receive service priority proportional to
waiter count. One active operation owns one service entry; caller publication is
separately resumable. Avoid polling all blocked callers as an unexamined scheduler
cost. Event traces establish actual overlap rather than assuming simultaneous arrival.

Positive workload: explicit alternatives bind distinct output tags, then perform
alpha-equivalent nontrivial binding equations; all answers remain distinct. Opposed
controls: delayed requests after completion, unique deep constructors, cheap shared
identity terms. Proposed small grids fanout 2/8 and depth 8/64 need registration.

Falsifying cases: identical raw operands under incompatible environments; repeated
versus distinct variables; transitive occurs paths; alpha-renamed allocated domains;
output/residual aliases into patched variables; shared success followed by one caller
failure; different occurrence IDs/propagation histories; constructor/guard enablement;
late arrivals; large shared work beside a small answer; recursive finite prefixes.
A deterministic service test must submit two requests before either executes.

Gate projected direct and sharing against all 64 independent source fixtures and
projected state correspondence, not just final outputs. Record arrivals, computed
operations, joins, memo hits, all interface/solver/fan-out work, retained entries and
waiters, source steps, failure/raw/unique/exhaustion and publication indices. Register
runtime/storage only after a resource pilot. Reduced solver work without reduced
complete cost establishes a mechanism, not an economical implementation. Equal memo
benefits support ordinary reuse; reduced active-only retention versus missed late hits
is a tradeoff to measure. No owner decision is currently required for this experiment.

## Representation control refinement

Current map_term/resolve/Rename rebuild constructors, and ordinary JSON tree fixtures
cannot express input DAG identity. A broker semantic gate can proceed on current
trees; its first performance comparison needs a separate rebuild/preserve factor.
Preserve an original immutable constructor when all transformed children retain
identity, with traversal-local identity memoization for repeated references. Do not
cache resolution across changing substitutions. Enable the same sound identity
shortcut in both direct and shared solvers. Charge traversals even when no allocation
is required; preserved roots alone do not make lookup constant-time.

Use deterministic topology-aware constructors for shared-DAG versus separately built
but structurally equal controlled operands. Charge preparation and provide their tree
denotation to the independent oracle. Existing application JSON remains uninterned;
report its actual topology rather than silently manufacturing sharing. Hash-consing
is a further representation experiment, not part of an unnoticed baseline upgrade.

Root identity across contexts is not sufficient for nonground operation equality.
A future dependency-metadata key must charge preparation, binding reachability and
alias comparison, with exact structural fallback; groundness certification can justify
a cheaper ground path. Register that follow-up if full projection dominates. The
minimal representation factor can accompany rather than block the semantic broker
gate, but must precede any shared-input performance claim.

## Bounded primary-source follow-up

Existing T011-source-resolution, T009-coverage, T018-composition-and-learning and
T016-asynchronous-symbolic-scheduler already cover projection, caller constraints,
CHR consumable-store caveats and frozen-call scheduling. The additional search adds
two protocol controls rather than another broad tabling survey.

Chico de Guzmán, Carro and Warren, [Swapping Evaluation, §§3–5](https://software.imdea.org/~mcarro/Material/Tabling/swapping_tabling_published.pdf),
distinguish generator completion, consumer suspension/resumption and retention under
conservative completion boundaries. Inference for this nonrecursive finite service:
use running (accept joins), complete/delivering (existing waiters own result references,
no active-only lookup), and released as distinct states. A late request during delivery
starts a new active-only computation; completed memo may reuse it. Reclamation must
not depend on later caller CHR execution or whole-search exhaustion. Test an unequal
patch-size pair plus unrelated progressing work and record result/waiter retention.
Their recursive SCC/fixpoint and stack mechanisms do not supply a fairness theorem
for this broker. Explicitly prohibit equality jobs from recursively requesting other
broker operations in this first gate.

Areias and Rocha, [Towards Multi-Threaded Local Tabling Using a Common Table Space,
§§2.2–3.3](https://arxiv.org/pdf/1210.2282), share table structures while retaining
independent thread-local evaluation and private subgoal information. Consequently,
shared key/result storage with independent computation is a credible matched control.
Count physical service starts/advances separately from shared nodes, entries and
consumers. The paper's multithreaded timings and dependency coordination do not transfer
to a sequential equality broker with one terminal MGU/failure. No retrieved primary
result replaces our patch correctness, bounded delivery or fairness checks.

## Semantic argument to discharge at the gate

Let S be a caller's acyclic finite substitution and E its next equation. Resolve E
under S, then apply a bijection rho from the remaining free representatives to fresh
canonical hole IDs. A solver returns failure or an MGU mu of rho(resolve_S(E)).
The caller patch is rho-inverse(mu) on those originally free representatives only.
Its composition with S has the same solutions as S together with E: resolution
preserves S's equations; bijective renaming preserves unifiability; applying an MGU
imposes exactly the remaining equation. Keys outside the projection remain unchanged,
while aliases into projected representatives observe the extension through S.

The executable obligations are acyclic input resolution, joint (not per-operand)
renaming, exact free-representative inverse mapping, no solver-introduced holes,
no overwrite of bound caller keys and complete MGU/failure publication. A triangular
solver substitution need not be eagerly idempotent, but its resolved denotation
must match the independent oracle. This argument does not permit discarding caller
residuals or histories, merging source alternatives, or resuming an active caller
against a changed S. Scheduling/completion and resource behavior need separate gates.

## Preserve attribution when integrating the scheduler

A completed-source-step checker can validate projected equality on all 64 existing
cases independently of search scheduling. For an interleaved broker, distinguish
exhausted-case answer-set checks from finite prefixes: a changed service schedule
can change which valid synthesis answers occur first. Do not silently require an
incidental answer order or weaken a full finite-answer check. Register prefix
membership/witness and liveness obligations separately before execution.

The current NetEquality bridge copies and scans the complete service counter map after
every one-action advance (five categories for scan, six for count). That is concrete
administrative work included in wall time but is not another logical net action.
It helps explain why action totals alone are insufficient, without attributing the
measured time difference to this path absent a timing ablation. A broker that advances
an owned service by a larger quantum changes this interface cost even without reuse.
Projected no-reuse and sharing controls must use the same service advance and reporting
protocol. Keep scheduling, projection, representation and computation reuse identifiable.
