# Stronger compiled controls are correct, but still enumerate the choices

Prepared specialization and pure-prefix lowering execute the choice/check source correctly. Both preserve its explicit search tree. This qualifies stronger controls for a lifecycle comparison; it does not establish that compilation or direct solving cannot eliminate the choices.

## What each existing mechanism does

| Control | What executes on this source | What remains |
|---|---|---|
| Inferred prepared specialization | Specialized rule matching/body instructions execute, including every consuming application. | All explicit choice splits, failing leaves and source applications. |
| Pure-prefix lowering | The compiler identifies `coin/1` as a closed private pure prefix and inlines its equations and OR bodies into an entry rule. | Explicit OR search and subsequent checks; creating the entry body adds engine work. |
| Reusable prefix artifact | The emitted entry/rules are prepared once for an ordered query predicate/arity signature. Changed argument terms and output identities are supplied later. | The same OR search. Changed predicate count/order needs a different artifact; reuse is not automatically free across changing shapes. |
| Existing sealed recursive compiler | Rejects this source because its certificate requires exactly two rules. | Broader contextual/effectful lowering is untested by that rejection. |
| Existing resource-fusion compiler | Finds no certified private resource boundary in these rules. | General resource-aware solving is untested by that rejection. |

The stream controls qualify inferred specialization. They do not have the closed acyclic private prefix required by this prefix compiler. No ordinary execution is silently labeled as successful prefix lowering.

## Measured work, not timing

Depth eight, resource present, successful oldest-first source; newest-first has identical explicit-engine counts:

| Control | Explicit splits | Failed leaves | Counted engine steps (`source_steps`) | Rule applications | Rule dispatches |
|---|---:|---:|---:|---:|---:|
| Scan | 255 | 255 | 11,770 | 2,042 | 6,905 |
| Inferred specialization | 255 | 255 | 9,727 | 2,042 | 6,905 |
| Fresh prefix lowering | 255 | 255 | 12,276 | 1,788 | 6,651 |
| Reusable prefix artifact | 255 | 255 | 12,276 | 1,788 | 6,651 |

The prefix replaces255 coin applications with one entry application, saving254 applications. Its engine-step count nevertheless rises. Specialization executes all2,042 applications through its specialized path and lowers the engine-step count. These counters describe the implementations' serviced operations; they are not equal-cost units or evidence of a timing ordering.

Across registered arrival depths0/1/4/8, each control makes2^n−1 explicit splits. With a final success there are2^n−1 failed leaves; with explicit final failure there are2^n failed leaves. The analytical successful answer remains unique. Thus these optimizations do not remove enumeration in this source regime.

For the successful depth-eight alias stream, specialization lowers counted engine steps from409 to337 while preserving63 applications and eight splits. This supplies the earlier conditional-adverse source as a qualified stronger control too.

## Correctness, reuse and evidence

The [prospective gate](../registrations/S10-arrival-controls.md) covers112 ordinary source configurations:64 arrival configurations and48 stream configurations. It varies both arrival orders, size, resource presence, final failure and query-tag reversal. Complete answers agree with analytical expectations and the independent owned-syntax scalar evaluator. Aliases and residual multiplicity are checked jointly.

Six additional probes check missing tokens, duplicate tokens and shared coin variables for both arrival orders. Missing tokens preserve a pending finish constraint; duplicate tokens leave exactly one residual token. Shared coin variables invalidate incompatible assignments, and all transformed paths agree with the independent scalar result. These probes challenge resource behavior beyond the standard single-token source.

Four queries run through reused prefix artifacts, including changed work/payload values and freshly renamed input/output variables. Incompatible signatures are explicitly rejected. The same prepared artifact therefore supports changed terms and identities within its checked shape; this gate makes no cross-shape preparation-cost claim.

Two diagnostic executions agree byte-for-byte on376 work rows per run. A separate counter-free build agrees on every admission, split/failure endpoint and complete-answer assertion. Retired fork prefixes and terminal segments are counted once, using the engine's documented work ownership. All three bounded runs complete within the registered limits. Scoped Clippy and package formatting pass. No reference implementation or engine behavior changes.

[Raw runs, build commands and source/binary hashes](s10-arrival-controls/), the [runner](../../../research/chr-direct-conditional/examples/arrival_controls.rs), and the [bounded reproduction driver](../../../research/chr-direct-conditional/experiments/arrival_controls.py) provide the evidence. No comparative timings were collected in this gate.

The preceding [order lifecycle report](S08-order-lifecycle.md) now records the correct source-gate count of32 combinations per observer build. Its756 comparative processes and108 allocation-pair counts are unchanged.

## Next decision: charge the qualified controls before interpreting the sharing advantage

Register a complete lifecycle comparison under T078 with inferred specialization and both fresh/prepared prefix lowering on the qualified sources. Include the earlier conditional order controls, changing argument values within a signature, changed signatures requiring preparation, immediate and retained answers, failure and cancellation. Charge artifact construction, source/input setup, observation and all disposal. Include the adverse stream for candidates that actually accept it; report ineligibility separately. Native generation/compilation remains a distinct required comparison.

The strongest ready alternative is T079 effect certification. Its independent witnesses exist, but an additional executable beneficiary and a sound effect property still need implementation. These newly qualified compiled controls can now directly challenge the conditional source-level advantage at lower implementation cost. This justifies a bounded lifecycle package next, rather than inferring runtime from the work counters. Reconsider T079 at that package boundary; it remains required.

Direct resource-aware solving that eliminates the alternatives remains a distinct candidate. Neither this prefix compiler's retained OR tree nor another compiler's ineligibility resolves it. A hand-written generator of the analytical expected answer would only demonstrate an exact-source specialization; it would not qualify a general source-derived solver. The next lifecycle result must retain that distinction and cannot establish a whole-architecture winner.

T078 remains active. Broader coherent architectures, language tradeoffs, held-out challenges and unresolved mechanisms remain required; the research goal is not complete.
