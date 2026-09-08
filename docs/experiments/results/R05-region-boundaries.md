# R05 relation-region boundaries

Region specialization has two distinct correctness contracts. Eliminating generic matching, partner enumeration or propagation bookkeeping does not automatically authorize evaluating a relation earlier. Eight executable boundary tests now establish concrete accepted examples and counterexamples. They do not establish a general eligibility checker or a specialized executor; T037 remains active.

## A: specialize execution while preserving source scheduling

A specialized path must select the same enabled source rule and ordered occurrence tuple, retain nonbinding matching and guard behavior, execute the same ordered body with fresh locals, and preserve equality failure and explicit-choice multiplicity. Completion still requires the absence of pending work and enabled applications. Administrative service must make finite progress and source alternatives must receive the required service.

A useful initial class is predicates whose rules consume exactly one occurrence, with no kept heads. Compiled ordered dispatch can support multiple constructor cases and equality guards. This can eliminate general partner joins and propagation history for those calls. It does not eliminate occurrence identity, global priority or body ownership. Constructor and repeated-variable demands still require nonbinding tests and wake-ups. A sole distinct-variable catchall with no guard can additionally make those eligibility tests unnecessary. None of these claims requires all arguments ground or all recursion terminating.

The class is sufficient for a selected implementation experiment, not a proposed necessary language restriction. Multihead or propagating regions may qualify through a different correspondence certificate, as the bounded R04 finite-region work demonstrates. An optional checked declaration can request this analysis; it cannot authorize an otherwise invalid transformation.

The next implementation gate should demonstrate an actual specialized path on accepted recursive, nonground and contextual witnesses, preserving the source selection boundary. Existing ordinary engine execution of the witnesses is not evidence that machinery has already been eliminated.

## B: evaluate a region eagerly and eliminate internal arbitration

This requires additional evidence: surrounding effects must commute in the relevant observation model, exhaustion must not be confused with unresolved divergence, and finite answers must remain reachable with finite service. Predicate closure alone does not establish these properties. Failure is a branch-wide effect even if local term variables are disjoint.

A checked termination-and-commutation argument is one possible sufficient certificate. Termination may depend on a decreasing control argument while other arguments remain unknown. A stronger divergence-sensitive correspondence could admit more programs. Neither a blanket groundness requirement nor a bounded test cutoff substitutes for that argument. Raw alternative multiplicity requires a correspondence between histories, not merely equality of successful observation sets.

Inferred eligibility and checked optional declarations can retain unrestricted ordinary execution for programs outside the certified class. Mandatory restrictions instead exclude or require reformulating those programs and therefore need an explicit language decision. This investigation has not adopted such restrictions. The exact tradeoff to establish is which runtime responsibilities each checked property eliminates, including the cost of checking/linking and retaining the boundary with surrounding CHR.

## Executable boundary evidence

`research/chr-compiled/tests/region_boundaries.rs` uses ordinary generic global indexed execution plus an independently implemented scalar source oracle for every complete finite case. Expected adverse observations are independently asserted. Full answers preserve joint variable identity, residual multiset and raw multiplicity.

| Witness | Verified distinction |
|---|---|
| Unique p definition with shared variable | With rules q(a)→A, q(X)→B, p(X)→X=a and query q(Y),p(Y), source order produces B; moving p first produces A. Unique dispatch does not authorize effect reordering. |
| Constructor head on unknown input | A blocked nonbinding p(f(X)) head leaves p(Y) residual. Replacing its match by an equation binds Y and changes the full answer. |
| Repeated head variable | p(X,X) does not identify two independent query variables merely to fire. A binding lowering changes aliases and residuals. |
| Equality guard | Entailment of X=a is not an assertion that may bind X to make the rule eligible. |
| External propagation | Two equal-valued p occurrences are each observed before consumption in source order. Eager consumption loses those observations. |
| Equal OR arms | Two successful identical arms deliver two raw answers; a single arm delivers one. |
| Open constructor domain | Input p(c) remains residual when only p(a) and p(b) rules exist. Treating an assumed two-constructor signature as exhaustive turns it into failure. |
| Closed inlining examples | Three opaque argument shapes, including an unknown and a repeated-variable structure, preserve full answers and fresh locals under the tested entry-to-p inlining. There is no interfering surrounding context in these examples. |
| Recursive closed unfolding | Original and twice-expanded unary generators each produce the first eight expected unary answers with finite service. This is a concrete prefix check, not unrestricted unfolding correspondence. |
| Exhaustion versus divergence | Higher-priority p→p prevents q→Fail from running; eager failure exhausts. The test checks 10,000 continuing source steps; the recurrence supplies the unbounded nontermination argument. Equal empty finite-answer sets alone would miss this distinction. |

The eight test functions cover these witnesses in both counter configurations. Workspace tests, strict Clippy and formatting pass; [receipts](r05-region-boundaries/) are retained. Independent review corrected the interpretation of the closed inlining example and strengthened raw-multiset inequality and failure-event checks. No comparative timings were run.

## Literature boundary and architectural disposition

Gabbrielli, Meo, Tacchella and Wiklicky's [Unfolding for CHR programs](https://arxiv.org/abs/1307.0679) separates the unfolding operation from conditions permitting safe replacement, and treats preservation of confluence and termination under additional conditions. That distinction supports asking for an explicit contract; it does not transfer a theorem automatically to this repository's selected priority, raw-multiplicity and progress observations.

Betz and Frühwirth's [Linear-Logic Based Analysis of CHR with Disjunction](https://arxiv.org/abs/1009.2900) provides an analytical relationship between operational and logical meaning. Logical correspondence remains a research basis, not an automatic certificate for every observable scheduling and multiplicity property used here.

T037 now selects contract A's executable specialized path first, while retaining B as a separately certified possibility. This can determine whether general machinery is avoidable without changing language behavior. It is more valuable than extending a finite solver's domain before knowing its contextual eligibility or tuning conditional runtime overhead again. The broad architecture goal remains active; no general region eligibility, language adoption or cost superiority is claimed.
