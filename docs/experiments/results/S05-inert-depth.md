# Deeper call reuse saves real execution and exposes an owned-key memory limit

Separated call reuse avoids hundreds of machine transitions on deeper inert callers, while readable and late-bound callers retain all the work. Owned whole-state keys fail the 1 GiB process bound on one deep duplicate-residual case; compact keys and separated execution complete it. These results identify useful and adverse inputs for the missing cost comparison, not a speed winner.

The [original registration](../registrations/S05-inert-depth.md) extends the existing recursive source from depths 0/4 to 32/128. The source already performs one recursive rule application per depth unit; no executor or reference semantics change is needed. All six families, identical/distinct caller tags and two changing query identities remain represented.

## The useful work is now substantial

Counts below use depth 128, distinct callers and the first query identity. The other query identity and independent diagnostic repeat agree. An executed transition runs the source machine; a hit reuses a cached transition but still schedules and transports the derivation.

| Caller/source condition | Direct executed | CompactLive executed | Separated memo executed | Memo hits / key requests |
|---|---:|---:|---:|---:|
| Inert ground caller | 783 | 783 | 395 | 388 / 396 |
| Readable caller | 787 | 787 | 787 | 0 / 787 |
| Late-bound variable caller | 789 | 789 | 789 | 0 / 789 |
| Same predicate name, different readable arity | 783 | 783 | 395 | 388 / 396 |
| Duplicate inert residuals | 1,295 | 1,295 | 651 | 644 / 652 |
| One branch fails early | 396 | 396 | 396 | 0 / 396 |

At depth zero, inert separation saves only four transitions, from 15 to 11. At depth 128 it saves 388. The gate therefore supplies a real scale of avoided recursive execution for the next lifecycle comparison. It also retains zero-benefit cases instead of treating all callers as eligible reuse.

**Identical callers are a control for unnecessary separation.** At depth 128 with the same inert caller tag, whole-state AlphaLive, CompactLive and separated memo each execute 393 transitions and reuse 390. Separation has no additional work-count gain there. Its distinct-caller result comes from excluding irrelevant observations from the active key, not from universally improving an already matching whole-state key.

**Avoided execution does not eliminate derivation service.** The inert memo case still performs 783 logical transitions, including its 388 cache hits. It requests 396 keys and retains 395 table nodes. Separated uncached execution uses three reusable node slots but executes all 783 transitions. Those slots are not a measurement of all machine-arena or output memory. Key construction, detached residual export, edge transport and observation can still outweigh the saved execution.

## The memory obstruction is representation-specific within these controls

The original batch aborts after 392 completed control rows. A [fresh-process probe](s05-inert-depth/isolation-probe.json) reproduces allocation failure on distinct callers with depth-128 duplicate residuals in AlphaLive whole-state mode. The 1 GiB address-space limit is unchanged. This rules out prior batch retention as a necessary explanation; it does not measure exact peak requested heap or RSS.

The [registered isolation follow-up](../registrations/S05-inert-depth-isolation.md) runs every original source/mode group in its own process. Of 960 processes, 956 complete and four fail: exactly that whole-state case in both repetitions of both metrics configurations. The successful processes supply 1,912 complete control rows. No partial failed run is reported as a completed work count. All successful diagnostic repeats agree exactly and all plain counters are zero.

**CompactLive completes the same source with the same state-normalization policy.** Whole-state AlphaLive exports ground terms into owned tree keys; CompactLive exports compact ground identities before the corresponding live-history normalization. The source creates two ground residuals at every recursive step. Repeated owned export therefore retains many copies of growing residual structure across state keys; distinct callers also prevent the whole-state keys from merging their futures. Separated execution detaches the unreadable ground observations from those active keys. The source and key implementations explain a plausible growth mechanism, and the compact/separated controls demonstrate that this capacity failure is avoidable without changing the source answers. Exact byte attribution remains required.

This is an adverse result for the owned-key implementation under the registered resource bound. It does not reject state reuse, call separation or deeper source execution. Raising the limit would not answer why the stronger representations complete within it.

## Correctness and evidence limits

Every successful process compares complete raw answers with the independent scalar evaluator and FIFO delivery with Direct. Analytical answer counts are two per query except early failure, which has one. The checks retain fresh-variable aliasing, caller observations, duplicate residual multiplicity and variable-dependent outputs. Each mode cancels a separate query after one service call, then reuses prepared rules for two changing query identities. Completed answers remain valid after engine and preparation disposal. Caches remain query-owned; this is not cross-query cache reuse.

The [work runner](../../../research/chr-reuse/examples/inert_work.rs) has deep-source tests in both builds. A CLI selector isolates resource ownership per candidate. Its current implementation uses an explicit selector parameter; the archived matrix runner received the same selector through the CLI. The source snapshots preserve that distinction. Strict Clippy and semantic validation are recorded alongside the frozen evidence.

The [original frozen attempt](s05-inert-depth/freeze.json), [isolated freeze](s05-inert-depth/isolated/freeze.json), [raw isolated receipts](s05-inert-depth/isolated/runs/), and [audit](s05-inert-depth/isolated/audit.json) preserve completed and failed executions. The [auditor](../../../research/chr-reuse/experiments/audit_inert_depth_isolated.py) checks source/binary identities, feature closure, every configuration, exact successful repeats and all allocation failures. No wall-time or allocation-traffic ranking is inferred from these counters.

## Next complete the causal cost comparison

T075 remains active for extraction/key/replay attribution and ordinary-allocator lifecycle costs on these qualified deeper sources. Keep Direct, CompactLive and separated uncached controls; qualify existing competent compiled/inferred source execution where applicable before attributing a gain solely to reuse. Preserve the owned-key capacity obstruction as a failed cell, and investigate its growth with bounded allocation attribution rather than treating the compact control as proof of an exact byte saving.

This follow-through takes precedence over integrated token-independent eligibility and demand discovery/copy repair because it can now test whether substantial avoided execution repays recognition and transport. Those distinct implementation trials remain required. Conservatively count the initial screen and isolated resource follow-up as packages two and three after the full post portfolio review: the next cost/attribution result or obstruction must trigger full portfolio review. Broader call relevance, failed futures, fresh identities, eviction and sustained consumers remain open; this gate does not establish architecture or research completion.
