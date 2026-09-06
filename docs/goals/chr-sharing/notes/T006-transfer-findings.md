# Second research checkpoint: conditions for sound shared execution

Published CHR semantics already combines propagation history with explicit disjunction. The remaining work is to adapt that reference precisely and establish when several alternatives may share a step. This pass also identifies two further design opportunities: schedulable primitive operations and compiler-inferred storage reuse.

No architecture or language change is selected here. The investigation covers competing mechanisms and the source properties each needs. It contains source inspection and semantic analysis, not project measurements or a CHR sharing proof.

## What the reference semantics can reuse

De Koninck, Schrijvers and Demoen's *Flexible Search Strategies in Prolog CHR* defines theoretical token-aware CHR with disjunction in §3.1/Table 1. Each alternative has pending goals, identified constraint occurrences, built-ins, propagation history and a fresh-ID counter. Splitting preserves this state in both arms; subsequent derivation changes one alternative. The published collection is a set, which requires adaptation for our duplicate-preserving internal alternatives. [CW447, 2006](https://www.cs.kuleuven.be/publicaties/rapporten/cw/CW447.pdf)

The reference therefore needs a focused specification, rather than an invented combination of unrelated calculi. It must define finite-tree equality and guard entailment, distinct alternative occurrences, rule-local freshness, residual answers and quiescence. Shared runtime identities must correspond to fresh identities inside each represented alternative. Copying the same mathematical counter into two independent states does not permit cross-alternative aliasing in an implementation.

There is also an adaptive CHR disjunction semantics that restores stores and histories through justifications. It uses refined execution, and its inspected conclusion describes correspondence propositions as ongoing work. We can investigate its restoration mechanism without treating it as a proved implementation of our unordered baseline. [Wolf, Robin and Vitorino, §§3–4](https://doi.org/10.1007/978-3-540-92243-8_3)

## Fairness creates a language and compiler question

A published Curry implementation explicitly demonstrates that both breadth-first and depth-first queues can stall on deterministic evaluation. Its fair mode uses goroutines; shared updates require locking. The discussion mentions a task cap without establishing fairness under that cap. This confirms that queue order alone does not settle the issue. [Böhm, Hanus and Teegen, PPDP 2021, §7.2](https://www.michaelhanus.de/papers/PPDP21.pdf)

For this project, running an alternative until quiescence can have the same problem. Candidate schedulers need finite work steps and an admission policy that lets a successful alternative progress even when others diverge. A shared computation must not make unrelated alternatives wait for a branch-specific divergent calculation.

This gives a concrete design choice to investigate: require primitive operations to terminate, make them resumable, or certify a restricted fragment for which fairness can be promised. Compiler analysis may establish adequate properties locally. A global restriction would limit host operations and may require programmers to express long computations as CHR steps. Finite terms ensure that a particular traversal can finish; they do not establish a uniform latency bound or termination of an arbitrary guard implementation.

This concerns execution of pure guards, not permission for guards to mutate state. The existing prohibition on unification equations in guards remains the baseline.

## Reuse needs more than equal arguments

TCHR makes the dependency problem concrete: even ground keys can connect a call to constraints outside its visible variable graph. Its table encoding can preserve occurrence history; replay without history requires set semantics. Projection must preserve inconsistency that has not yet been detected. [Schrijvers, Demoen and Warren, §§5–7](https://arxiv.org/pdf/0712.3830)

There are three substantive ways to obtain a reusable subcomputation: include its relevant environment in its identity, prove isolation through analysis, or require an interface that confines interactions. These alternatives have different costs. Explicit interfaces can make reuse easier to establish but restrict joins and shared resources. Analysis preserves flexibility but can overestimate dependencies. Environment-sensitive identity can become as large or unstable as the store it is intended to abstract.

Persistent or set-like constraint families are consequently worth investigating for a specific reason: they can reduce occurrence/history obligations during replay. This does not establish that all CHR predicates should be sets. Programs that use duplicate occurrences as resources need their multiplicity preserved. The question is whether suitable families can be certified locally, and how they interact with consumable constraints.

ACD rewriting provides a related language extension: rules can use a surrounding conjunctive context without explicitly distributing it through the term. Its published CHR correspondence restricts guards and built-ins; adding an underlying solver is discussed as an extension, not included in that theorem. This is evidence for a useful context mechanism with a precise transfer boundary. [Duck, Stuckey and Brand, ICLP 2006, §§3–5](https://www.comp.nus.edu.sg/~gregory/papers/iclp06.pdf)

## Local rewriting does not require only one net discipline

Multiport interaction nets retain local replacement while allowing competing active pairs. Mazza's operational correspondence is for a specified core π-calculus with replication. Finite implementations and broader source constructs require further work; this is not a CHR compilation theorem. [CONCUR 2005, §§2–3](https://www.lipn.fr/~mazza/papers/mINSAndConcurrency-CONCUR05.pdf)

This keeps a richer-net route in consideration alongside restrictions that permit ordinary interaction nets. The comparison must account for routing and synchronization, atomic multiheaded consumption, and the separation between committed competition and explicit alternatives. Losing strong confluence does not by itself mean losing local rewriting. Conversely, locality does not establish inexpensive joins or preservation of all explicit alternatives.

Compiler-inferred storage reuse is a separate opportunity. Mackie and Sato classify interaction rules by RHS size and annotate reused cells and connections. With their fixed-size layout, a two-node active pair can provide cells for a RHS of at most two nodes. Fixed-size cells have a space cost; benchmarks and broader scheduling claims are deferred. [TERMGRAPH 2016, §§3–7](https://arxiv.org/pdf/1609.03641)

A compiler could recognize eligible rules without requiring every program to meet a global resource bound. In our setting, however, a logical occurrence consumed under choice condition A may remain live under B. Reusing its physical cell requires exclusive ownership or an update representation that preserves both meanings. Allocation savings alone do not meet the requirement to perform common computation once.

## Conditional representation has its own proof boundary

The Formula Choice Calculus supplies denotationally sound rules for moving and merging choices labelled by Boolean formulas. Its configuration-to-object semantics and equivalence theorems support representation transformations; they do not supply CHR transitions, dynamic choice generation or a cheap normalization algorithm. [Hubbard and Walkingshaw, FOSD 2016, §§2–4](https://eric.walkingshaw.net/files/pubs/2016/fosd16-formula-choice-calculus.pdf)

For a CHR representation, choice identities would remain internal. Merging equal values must retain enough information to represent distinct alternatives and their residual stores. Equal values under two configurations do not prove that the alternatives have identical futures or that an active constraint can be ignored. This applies equally to a term graph carrying named choices and a store whose entries carry Boolean conditions.

## Learned consequences are another form of shared work

SMCHR stores clauses generated by CHR rules and reuses them during SAT search, avoiding some repeated propagation. It uses set semantics, range-restricted rules and Boolean reification; disjunction is limited to initial goals. Its soundness result concerns UNSAT, not complete answer enumeration, and its unification is not general finite-tree unification. Experiments show benefits on search workloads and overhead on others. [Duck, SMCHR, §§3–8](https://www.comp.nus.edu.sg/~gregory/papers/smchr.pdf)

This warrants a separate candidate: reusable logical consequences, possibly within a certified solver fragment. The language costs are concrete. Duplicate resources need explicit identities under set semantics, and a rule that introduces a fresh existential variable does not satisfy range restriction. The proof obligation is that a learned consequence remains valid wherever reused despite consumption, aliasing and propagation history. User-visible reification would be a separate language proposal; internal choices can remain opaque.

## Candidate obligations to compare next

These are analytical obligations derived from the project semantics, not performance rankings.

- **Conditional stores and assumption supports:** show how one occurrence can have different lifetimes and propagation permissions across alternatives. Account for condition manipulation, dynamic identities, and inconsistent contexts.
- **Memoized graph evaluation:** establish complete dependency identities for CHR joins and aliases, and prevent a branch-specific demand from blocking shared work needed elsewhere. Expression identity is useful evidence of common computation, but does not identify every environmental dependency.
- **Ordinary or richer local nets:** state the source property or encoding invariant that makes a join local, then account for administrative steps and conditional ownership. Compare compiler inference, optional contracts and global restrictions separately.
- **Tabling and solver interfaces:** define reusable summaries, reinstatement of caller conditions, and residual consistency. Compare discovery of repeated subproblems with reuse of an already-shared computation. Any change from multisets to sets must be stated explicitly.
- **Storage controls:** include copying with recomputation and storage sharing without execution sharing. These test total cost; they are not substitutes for the requested computational sharing.
- **Learned logical consequences:** establish a fragment where clauses remain valid across alternatives and distinguish reused propagation from executing an entire common continuation once. Account for restrictions on fresh variables, resources and dynamic disjunction.

One common discriminating case is an occurrence consumed in one alternative, retained in another, and participating in propagation with a partner whose binding later changes. Each candidate must either describe the permitted behavior under its own semantics or identify the restriction that excludes the case and assess any feasible reformulation. This case tests expressibility and correctness; it is not evidence of representative performance.

## Research status and next gate

C01 (semantic fidelity) now has a direct token-aware disjunction reference. C05 (answers) still needs the project's residual observation and recognized equivalence specified. C02–C04 (space, work sharing and net benefit) remain unmeasured. C06 (language tradeoffs) now includes schedulable primitives and inferred allocation reuse, as well as the first checkpoint's six opportunities.

The next task is a comparative semantic specification: a duplicate-preserving baseline reference, explicit semantics for candidate language changes, shared-work obligations for each family, and a comparison of inferred properties versus mandatory restrictions. A mechanism may require correspondence with several reference steps. These questions block correctness and fair comparison, so they precede prototypes. They do not require the owner to adopt a restriction now.

Coverage remains open. Targeted citation expansion resolved specific gaps, but systematic forward searches and the protocol's two successive rounds without a new family have not been completed. Original 1997/1998 texts remain access gaps; later primary definitions establish the narrower token/disjunction fact. The additional SMCHR inspection identified conflict learning as a distinct family; bounded-space type systems and static distribution analysis remain uninspected leads. M1 must not be marked complete from this checkpoint alone.

See [the retrieval and source record](T006-source-record.md) and [the first checkpoint](T003-research-findings.md).
