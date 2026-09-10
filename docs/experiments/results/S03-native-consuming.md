# Native execution preserves consuming effects in an admitted ground fragment

Source-derived native rule functions now preserve competing consumption, kept heads and branch-local effects in the tested ground programs. Complete residuals and raw multiplicity agree with independent occurrence semantics. This qualifies a bounded compiler path; it does not establish general occurrence identity, propagation or non-ground CHR execution.

## Actual source operations, rather than answer generation

The [compiler](../../../research/chr-hvm/consuming/compiler.py) generates a native function for each source rule. It checks head availability, consumes the removed heads, introduces the body and restarts source-priority selection. A source disjunction becomes a native superposition of successor executions. The compiler neither runs a search at preparation nor enumerates expected answers or reachable states.

The store is represented by occurrence counts. That representation is justified for the admitted fragment: constraints have no arguments, guards and propagation are absent, and every rule consumes an occurrence. Identical ground occurrences cannot be distinguished by these rules. Combined kept/removed multiplicity prevents one occurrence from occupying two head positions. This argument does not extend to propagation histories or identity-bearing values.

Each body arm must not increase total store size, and the initial store contains at most64 occurrences. Thus native32-bit arithmetic cannot overflow a count in this fragment. These are compiler admission conditions, not recommended language restrictions. The language-design cost of requiring such a fragment globally would be substantial and has not been proposed or measured.

## Results that distinguish resource semantics

| Source question | Required outcome | Native result |
|---|---|---|
| `a, token` and `b, token` compete for one token | Only the first source-priority consumer fires; the other task remains | Reversing rule priority reverses the winner |
| A rule removes two `token` heads | One token is insufficient; two distinct occurrences are necessary | Full residual multiplicity agrees |
| A rule keeps one token and removes another | A single occurrence cannot fill both head positions | With three tokens, two firings leave one token and two receipts |
| A kept token is observed before a later consuming rule | The observation survives consumption | The complete residual includes the observation |
| A choice occurs after consumption | Alternatives inherit the consumed state, not another usable copy of the resource | Binary and equal-arm branches preserve the expected residuals and raw counts |
| A second birth exists only in one arm | There are three histories, not four unconditional assignments | Nested choice returns three complete outcomes |
| A downstream consuming rule fails | The entire affected branch disappears | Failure filtering agrees with the source oracle |
| One source branch loops while another consumes and finishes | A finite complete residual appears while work remains | The native and reference paths both expose the finite sibling |

Ten families vary token counts0/1/2/3 and both query orders, producing80 registered configurations. Some order reversals produce identical inputs; they are controls, not independent semantic diversity.

## Independent checks and the diagnosed cutoff

The primary oracle uses concrete list positions as distinct occurrences. It searches source-priority head tuples with distinct indices, removes only selected occurrences and enumerates disjunctive successors. It does not use the compiler's count-vector matcher. Deterministic repeated-store loops are recognized separately from completed answers.

A research-side adapter constructs the same owned `Rule` and `Query` data and runs the unchanged reference engine. All80 reference runs agree on unique residual sets, total raw completions and exhaustion. The reference API deduplicates returned answers, so per-answer raw weights are established by the separate occurrence oracle, not inferred from reference totals alone. No reference implementation files or dependencies change.

The initial native matrix reaches its1024-call limit on a finite nested-choice case. Sizing completes the zero-token case in1804 calls and the three-token case in2875. The prospectively corrected matrix uses4096 calls uniformly. All56 initially completed records replay exactly; the cutoff's1024 service events match the prefix of the later completed execution. The bound adjustment changes no source operation or reducer algorithm.

The final run has320 native executions:80 configurations at transition quotas1/8, repeated twice. All160 paired records replay exactly. Another160 diagnostics-off runs preserve complete answers and scheduler states. All320 cancellations at0/1/4/16 calls match uninterrupted service prefixes, including pending roots and answer prefixes. Seventy-two terminating configurations agree with ordinary pinned native collapse. Together with the80 reference runs, these are952 successful comparison processes.

The compiler explicitly rejects propagation, guards, growing bodies, an unknown predicate and an oversized query. A separate native namespace test reports `ChoiceLimit` as an incomplete outcome. It is neither a successful CHR answer nor a failed branch. The Rust adapter builds and passes scoped Clippy with warnings denied.

## Native choices and remaining costs

Successor branches carry binary-tree birth identifiers: a birth at identifier L gives its children2L and2L+1. The tested source labels stay below2^23; the compiler’s ordinary value-copy requests use a separate higher range. The observer also uses source-labelled projection copies, and backend administrative superpositions are not necessarily source births. This avoids the earlier collision examples on the registered sources and preserves inactive-birth multiplicity. Exhaustion of the source range is explicit. General label reuse, unbounded freshness and lifetime management remain unresolved.

The generated native functions retain a serial rule-selection responsibility. Native labelled copying carries successor state across alternatives; this is not a distributed resource-claim implementation. Thousands of native service calls can correspond to only a few source applications in these examples. That is a reason to account for generated dispatch, copying and observation in a later complete-cost study, not a timing comparison or an inherent lower bound.

No preparation, compilation, allocation, RSS or execution-time winner is claimed. The current compiler also lacks a reusable prepared-query interface. Native weak-reducer traversal, reference instantiation, helper operations and final printing retain size-dependent service costs. These are necessary next measurement and implementation questions before any architectural conclusion.

## Next boundary: identities that counts cannot replace

T080 remains active. The next source gate must make occurrence/value identities observable: non-ground aliases, later binding and propagation histories cannot be represented by this ground count quotient. Establish an explicit identity-bearing native source representation and compare its serial effect owner with the obligations of local claims/commits. Include kept-head interference, incompatible consumption, off-output failure and cancellation during pending effects.

This is the third package since native work was selected: leaf service, structured observation and consuming ground correspondence. The next package must address that distinct identity/effect boundary, followed by the required breadth review against broader reuse, integrated execution and finite solving. More ground examples alone would not qualify a general CHR architecture.

The [current cycle](../next-cycle.md) retains those alternatives and complete lifecycle, coherent architecture and held-out comparisons. The research goal remains active.

## Retained evidence

[Registration](../registrations/S03-native-consuming.md), [driver](../../../research/chr-hvm/consuming/gate.py), [compiler](../../../research/chr-hvm/consuming/compiler.py) and [reference adapter](../../../research/chr-cases/examples/native_ground_reference.rs) define the experiment. The native observer derives from the frozen structured observer with only its accepted call-limit guard widened; the matrix requests4096 calls. The transition quotas,2 CPU seconds,3 wall seconds and96GiB virtual-address allowance are unchanged. Reference runs have100000 steps and5 wall seconds.

[Validation and hashes](s03-native-consuming/validation.json), [source cases and complete expectations](s03-native-consuming/sources.json), [reference observations](s03-native-consuming/references.json), [native runs](s03-native-consuming/runs.jsonl), [diagnostics-off runs](s03-native-consuming/counter-off.jsonl), [cancellation prefixes](s03-native-consuming/cancellation.jsonl) and [ordinary native controls](s03-native-consuming/controls.jsonl) preserve the comparison. The [initial limit record](s03-native-consuming/initial-limit.jsonl), [sizing](s03-native-consuming/bound-sizing.json) and [bound audit](s03-native-consuming/bound-audit.json) preserve the cutoff diagnosis. [Admission outcomes](s03-native-consuming/admission.json) and the [namespace-boundary check](s03-native-consuming/choice-namespace-bound.json) retain explicit limits.
